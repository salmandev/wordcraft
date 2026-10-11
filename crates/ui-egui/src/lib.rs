/// Platform services injected by the host (file dialogs, file I/O).
#[derive(Default)]
pub struct Services {
    /// Pick a file to open; the argument says what for (`document`, `picture`).
    pub pick_open: Option<Box<dyn Fn(&str) -> Option<String>>>,
    /// Pick a path to save to, given a suggested name.
    pub pick_save: Option<Box<dyn Fn(&str) -> Option<String>>>,
    /// Desktop: show a native file dialog without blocking the UI thread (#94); used instead of
    /// `pick_open` / `pick_save` when set. The host sends the picked path (`None` when cancelled)
    /// through the returned channel once the user answers, and wakes the UI; a dropped sender
    /// counts as cancelled. The app shows one dialog at a time ([`file_dialogs`]).
    pub file_dialog: Option<Box<dyn Fn(FileDialogRequest) -> std::sync::mpsc::Receiver<Option<String>>>>,
    /// Web: open a file picker; the file arrives later through `inbox`.
    pub open_async: Option<Box<dyn Fn(&str)>>,
    /// Web: files (name, bytes) delivered asynchronously (picker, drag and drop).
    pub inbox: Option<Inbox>,
    /// Web: hand bytes to the browser as a download. An error means no download started (the
    /// browser can't tell the page whether the user then kept the file).
    pub download: Option<Box<dyn Fn(&str, &[u8]) -> Result<(), String>>>,
    /// Web: told whether the document has unsaved changes after each pass, for the browser's
    /// leave-page guard (`beforeunload` runs between frames and can't ask the app).
    pub on_dirty: Option<Box<dyn Fn(bool)>>,
    /// Hand PDF bytes to the system's print flow. Web: the browser's print dialog on that PDF (no
    /// download, no intermediate file). Desktop: the PDF opens in the system's viewer (a temporary
    /// file), where the user prints (#15). An error means nothing was opened.
    pub print: Option<Box<dyn Fn(&[u8]) -> Result<(), String>>>,
    /// Desktop: the current text on the system clipboard, if any. Ribbon/context-menu Paste and the
    /// keyboard paste path all pass the same payload to the engine, so external clipboard text works
    /// regardless of how the user invoked it.
    pub clipboard_text: Option<Box<dyn Fn() -> Option<String>>>,
    /// Desktop: the picture on the system clipboard, if any — egui's paste only carries text
    /// (#45). Read when Paste finds no text; see [`paste_picture`].
    pub clipboard_picture: Option<Box<dyn Fn() -> Option<paste_picture::ClipboardPicture>>>,
}
