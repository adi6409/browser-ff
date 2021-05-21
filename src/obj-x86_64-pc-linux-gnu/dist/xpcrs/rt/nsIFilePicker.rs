//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/widget/nsIFilePicker.idl
//


/// `interface nsIFilePickerShownCallback : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIFilePickerShownCallback {
    vtable: *const nsIFilePickerShownCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIFilePickerShownCallback.
unsafe impl XpCom for nsIFilePickerShownCallback {
    const IID: nsIID = nsID(0x0d79adad, 0xb244, 0x49a5,
        [0x99, 0x97, 0x2a, 0x8c, 0xad, 0x93, 0xfc, 0x44]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIFilePickerShownCallback {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIFilePickerShownCallback.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIFilePickerShownCallbackCoerce {
    /// Cheaply cast a value of this type from a `nsIFilePickerShownCallback`.
    fn coerce_from(v: &nsIFilePickerShownCallback) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIFilePickerShownCallbackCoerce for nsIFilePickerShownCallback {
    #[inline]
    fn coerce_from(v: &nsIFilePickerShownCallback) -> &Self {
        v
    }
}

impl nsIFilePickerShownCallback {
    /// Cast this `nsIFilePickerShownCallback` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIFilePickerShownCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIFilePickerShownCallback {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsISupportsCoerce> nsIFilePickerShownCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIFilePickerShownCallback) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIFilePickerShownCallback
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIFilePickerShownCallbackVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void done (in short aResult); */
    pub Done: unsafe extern "system" fn (this: *const nsIFilePickerShownCallback, aResult: i16) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIFilePickerShownCallback {

    /// ```text
    /// /**
    ///   * Callback which is called when a filepicker is shown and a result
    ///   * is returned.
    ///   *
    ///   * @param aResult One of returnOK, returnCancel, or returnReplace
    ///   */
    /// ```
    ///

    /// `void done (in short aResult);`
    #[inline]
    pub unsafe fn Done(&self, aResult: i16) -> ::nserror::nsresult {
        ((*self.vtable).Done)(self, aResult)
    }


}


/// `interface nsIFilePicker : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIFilePicker {
    vtable: *const nsIFilePickerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIFilePicker.
unsafe impl XpCom for nsIFilePicker {
    const IID: nsIID = nsID(0x9285b984, 0x02d3, 0x46b4,
        [0x95, 0x14, 0x7d, 0xa8, 0xc4, 0x71, 0xa7, 0x47]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIFilePicker {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIFilePicker.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIFilePickerCoerce {
    /// Cheaply cast a value of this type from a `nsIFilePicker`.
    fn coerce_from(v: &nsIFilePicker) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIFilePickerCoerce for nsIFilePicker {
    #[inline]
    fn coerce_from(v: &nsIFilePicker) -> &Self {
        v
    }
}

impl nsIFilePicker {
    /// Cast this `nsIFilePicker` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIFilePickerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIFilePicker {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsISupportsCoerce> nsIFilePickerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIFilePicker) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIFilePicker
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIFilePickerVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void init (in mozIDOMWindowProxy parent, in AString title, in short mode); */
    pub Init: unsafe extern "system" fn (this: *const nsIFilePicker, parent: *const mozIDOMWindowProxy, title: *const ::nsstring::nsAString, mode: i16) -> ::nserror::nsresult,

    /* void appendFilters (in long filterMask); */
    pub AppendFilters: unsafe extern "system" fn (this: *const nsIFilePicker, filterMask: i32) -> ::nserror::nsresult,

    /* void appendFilter (in AString title, in AString filter); */
    pub AppendFilter: unsafe extern "system" fn (this: *const nsIFilePicker, title: *const ::nsstring::nsAString, filter: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* void appendRawFilter (in AString filter); */
    pub AppendRawFilter: unsafe extern "system" fn (this: *const nsIFilePicker, filter: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* attribute AString defaultString; */
    pub GetDefaultString: unsafe extern "system" fn (this: *const nsIFilePicker, aDefaultString: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* attribute AString defaultString; */
    pub SetDefaultString: unsafe extern "system" fn (this: *const nsIFilePicker, aDefaultString: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* attribute AString defaultExtension; */
    pub GetDefaultExtension: unsafe extern "system" fn (this: *const nsIFilePicker, aDefaultExtension: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* attribute AString defaultExtension; */
    pub SetDefaultExtension: unsafe extern "system" fn (this: *const nsIFilePicker, aDefaultExtension: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* attribute long filterIndex; */
    pub GetFilterIndex: unsafe extern "system" fn (this: *const nsIFilePicker, aFilterIndex: *mut i32) -> ::nserror::nsresult,

    /* attribute long filterIndex; */
    pub SetFilterIndex: unsafe extern "system" fn (this: *const nsIFilePicker, aFilterIndex: i32) -> ::nserror::nsresult,

    /* attribute nsIFile displayDirectory; */
    pub GetDisplayDirectory: unsafe extern "system" fn (this: *const nsIFilePicker, aDisplayDirectory: *mut*const nsIFile) -> ::nserror::nsresult,

    /* attribute nsIFile displayDirectory; */
    pub SetDisplayDirectory: unsafe extern "system" fn (this: *const nsIFilePicker, aDisplayDirectory: *const nsIFile) -> ::nserror::nsresult,

    /* attribute AString displaySpecialDirectory; */
    pub GetDisplaySpecialDirectory: unsafe extern "system" fn (this: *const nsIFilePicker, aDisplaySpecialDirectory: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* attribute AString displaySpecialDirectory; */
    pub SetDisplaySpecialDirectory: unsafe extern "system" fn (this: *const nsIFilePicker, aDisplaySpecialDirectory: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute nsIFile file; */
    pub GetFile: unsafe extern "system" fn (this: *const nsIFilePicker, aFile: *mut*const nsIFile) -> ::nserror::nsresult,

    /* readonly attribute nsIURI fileURL; */
    pub GetFileURL: unsafe extern "system" fn (this: *const nsIFilePicker, aFileURL: *mut*const nsIURI) -> ::nserror::nsresult,

    /* readonly attribute nsISimpleEnumerator files; */
    pub GetFiles: unsafe extern "system" fn (this: *const nsIFilePicker, aFiles: *mut*const nsISimpleEnumerator) -> ::nserror::nsresult,

    /* readonly attribute nsISupports domFileOrDirectory; */
    pub GetDomFileOrDirectory: unsafe extern "system" fn (this: *const nsIFilePicker, aDomFileOrDirectory: *mut *const nsISupports) -> ::nserror::nsresult,

    /* readonly attribute nsISimpleEnumerator domFileOrDirectoryEnumerator; */
    pub GetDomFileOrDirectoryEnumerator: unsafe extern "system" fn (this: *const nsIFilePicker, aDomFileOrDirectoryEnumerator: *mut*const nsISimpleEnumerator) -> ::nserror::nsresult,

    /* attribute boolean addToRecentDocs; */
    pub GetAddToRecentDocs: unsafe extern "system" fn (this: *const nsIFilePicker, aAddToRecentDocs: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean addToRecentDocs; */
    pub SetAddToRecentDocs: unsafe extern "system" fn (this: *const nsIFilePicker, aAddToRecentDocs: bool) -> ::nserror::nsresult,

    /* void open (in nsIFilePickerShownCallback aFilePickerShownCallback); */
    pub Open: unsafe extern "system" fn (this: *const nsIFilePicker, aFilePickerShownCallback: *const nsIFilePickerShownCallback) -> ::nserror::nsresult,

    /* readonly attribute short mode; */
    pub GetMode: unsafe extern "system" fn (this: *const nsIFilePicker, aMode: *mut i16) -> ::nserror::nsresult,

    /* attribute AString okButtonLabel; */
    pub GetOkButtonLabel: unsafe extern "system" fn (this: *const nsIFilePicker, aOkButtonLabel: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* attribute AString okButtonLabel; */
    pub SetOkButtonLabel: unsafe extern "system" fn (this: *const nsIFilePicker, aOkButtonLabel: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* attribute short capture; */
    pub GetCapture: unsafe extern "system" fn (this: *const nsIFilePicker, aCapture: *mut i16) -> ::nserror::nsresult,

    /* attribute short capture; */
    pub SetCapture: unsafe extern "system" fn (this: *const nsIFilePicker, aCapture: i16) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIFilePicker {

    pub const modeOpen: i64 = 0;


    pub const modeSave: i64 = 1;


    pub const modeGetFolder: i64 = 2;


    pub const modeOpenMultiple: i64 = 3;


    pub const returnOK: i64 = 0;


    pub const returnCancel: i64 = 1;


    pub const returnReplace: i64 = 2;


    pub const filterAll: i64 = 1;


    pub const filterHTML: i64 = 2;


    pub const filterText: i64 = 4;


    pub const filterImages: i64 = 8;


    pub const filterXML: i64 = 16;


    pub const filterXUL: i64 = 32;


    pub const filterApps: i64 = 64;


    pub const filterAllowURLs: i64 = 128;


    pub const filterAudio: i64 = 256;


    pub const filterVideo: i64 = 512;


    pub const captureNone: i64 = 0;


    pub const captureDefault: i64 = 1;


    pub const captureUser: i64 = 2;


    pub const captureEnv: i64 = 3;

    /// ```text
    /// /**
    ///   * Initialize the file picker widget.  The file picker is not valid until this
    ///   * method is called.
    ///   *
    ///   * @param      parent   mozIDOMWindow parent.  This dialog will be dependent
    ///   *                      on this parent. parent must be non-null.
    ///   * @param      title    The title for the file widget
    ///   * @param      mode     load, save, or get folder
    ///   *
    ///   */
    /// ```
    ///

    /// `void init (in mozIDOMWindowProxy parent, in AString title, in short mode);`
    #[inline]
    pub unsafe fn Init(&self, parent: *const mozIDOMWindowProxy, title: *const ::nsstring::nsAString, mode: i16) -> ::nserror::nsresult {
        ((*self.vtable).Init)(self, parent, title, mode)
    }


    /// ```text
    /// /**
    ///   * Append to the  filter list with things from the predefined list
    ///   *
    ///   * @param      filters  mask of filters i.e. (filterAll | filterHTML)
    ///   *
    ///   */
    /// ```
    ///

    /// `void appendFilters (in long filterMask);`
    #[inline]
    pub unsafe fn AppendFilters(&self, filterMask: i32) -> ::nserror::nsresult {
        ((*self.vtable).AppendFilters)(self, filterMask)
    }


    /// ```text
    /// /**
    ///   * Add a filter
    ///   *
    ///   * @param      title    name of the filter
    ///   * @param      filter   extensions to filter -- semicolon and space separated
    ///   *
    ///   */
    /// ```
    ///

    /// `void appendFilter (in AString title, in AString filter);`
    #[inline]
    pub unsafe fn AppendFilter(&self, title: *const ::nsstring::nsAString, filter: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).AppendFilter)(self, title, filter)
    }


    /// ```text
    /// /**
    ///    * Add a raw filter (eg, add a MIME type without transforming it to a list of
        ///    * extensions).
    ///    *
    ///    * @param     filter   a filter taken directly from the accept attribute
    ///    *                     without processing
    ///    *
    ///    */
    /// ```
    ///

    /// `void appendRawFilter (in AString filter);`
    #[inline]
    pub unsafe fn AppendRawFilter(&self, filter: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).AppendRawFilter)(self, filter)
    }


    /// ```text
    /// /**
    ///   * The filename that should be suggested to the user as a default. This should
    ///   * include the extension.
    ///   *
    ///   * @throws NS_ERROR_FAILURE on attempts to get
    ///   */
    /// ```
    ///

    /// `attribute AString defaultString;`
    #[inline]
    pub unsafe fn GetDefaultString(&self, aDefaultString: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetDefaultString)(self, aDefaultString)
    }


    /// ```text
    /// /**
    ///   * The filename that should be suggested to the user as a default. This should
    ///   * include the extension.
    ///   *
    ///   * @throws NS_ERROR_FAILURE on attempts to get
    ///   */
    /// ```
    ///

    /// `attribute AString defaultString;`
    #[inline]
    pub unsafe fn SetDefaultString(&self, aDefaultString: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).SetDefaultString)(self, aDefaultString)
    }


    /// ```text
    /// /**
    ///   * The extension that should be associated with files of the type we
    ///   * want to work with.  On some platforms, this extension will be
    ///   * automatically appended to filenames the user enters, if needed.
    ///   */
    /// ```
    ///

    /// `attribute AString defaultExtension;`
    #[inline]
    pub unsafe fn GetDefaultExtension(&self, aDefaultExtension: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetDefaultExtension)(self, aDefaultExtension)
    }


    /// ```text
    /// /**
    ///   * The extension that should be associated with files of the type we
    ///   * want to work with.  On some platforms, this extension will be
    ///   * automatically appended to filenames the user enters, if needed.
    ///   */
    /// ```
    ///

    /// `attribute AString defaultExtension;`
    #[inline]
    pub unsafe fn SetDefaultExtension(&self, aDefaultExtension: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).SetDefaultExtension)(self, aDefaultExtension)
    }


    /// ```text
    /// /**
    ///   * The filter which is currently selected in the File Picker dialog
    ///   *
    ///   * @return Returns the index (0 based) of the selected filter in the filter list.
    ///   */
    /// ```
    ///

    /// `attribute long filterIndex;`
    #[inline]
    pub unsafe fn GetFilterIndex(&self, aFilterIndex: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetFilterIndex)(self, aFilterIndex)
    }


    /// ```text
    /// /**
    ///   * The filter which is currently selected in the File Picker dialog
    ///   *
    ///   * @return Returns the index (0 based) of the selected filter in the filter list.
    ///   */
    /// ```
    ///

    /// `attribute long filterIndex;`
    #[inline]
    pub unsafe fn SetFilterIndex(&self, aFilterIndex: i32) -> ::nserror::nsresult {
        ((*self.vtable).SetFilterIndex)(self, aFilterIndex)
    }


    /// ```text
    /// /**
    ///   * Set the directory that the file open/save dialog initially displays
    ///   * Note that, if displaySpecialDirectory has been already set, this value will
    ///   * be ignored.
    ///   *
    ///   * @param      displayDirectory  the name of the directory
    ///   *
    ///   */
    /// ```
    ///

    /// `attribute nsIFile displayDirectory;`
    #[inline]
    pub unsafe fn GetDisplayDirectory(&self, aDisplayDirectory: *mut*const nsIFile) -> ::nserror::nsresult {
        ((*self.vtable).GetDisplayDirectory)(self, aDisplayDirectory)
    }


    /// ```text
    /// /**
    ///   * Set the directory that the file open/save dialog initially displays
    ///   * Note that, if displaySpecialDirectory has been already set, this value will
    ///   * be ignored.
    ///   *
    ///   * @param      displayDirectory  the name of the directory
    ///   *
    ///   */
    /// ```
    ///

    /// `attribute nsIFile displayDirectory;`
    #[inline]
    pub unsafe fn SetDisplayDirectory(&self, aDisplayDirectory: *const nsIFile) -> ::nserror::nsresult {
        ((*self.vtable).SetDisplayDirectory)(self, aDisplayDirectory)
    }


    /// ```text
    /// /**
    ///   * Set the directory that the file open/save dialog initially displays using
    ///   * one of the special name as such as 'Desk', 'TmpD', and so on.
    ///   * Note that, if displayDirectory has been already set, this value will be
    ///   * ignored.
    ///   *
    ///   * @param      displaySpecialDirectory  the name of the special directory
    ///   *
    ///   */
    /// ```
    ///

    /// `attribute AString displaySpecialDirectory;`
    #[inline]
    pub unsafe fn GetDisplaySpecialDirectory(&self, aDisplaySpecialDirectory: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetDisplaySpecialDirectory)(self, aDisplaySpecialDirectory)
    }


    /// ```text
    /// /**
    ///   * Set the directory that the file open/save dialog initially displays using
    ///   * one of the special name as such as 'Desk', 'TmpD', and so on.
    ///   * Note that, if displayDirectory has been already set, this value will be
    ///   * ignored.
    ///   *
    ///   * @param      displaySpecialDirectory  the name of the special directory
    ///   *
    ///   */
    /// ```
    ///

    /// `attribute AString displaySpecialDirectory;`
    #[inline]
    pub unsafe fn SetDisplaySpecialDirectory(&self, aDisplaySpecialDirectory: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).SetDisplaySpecialDirectory)(self, aDisplaySpecialDirectory)
    }


    /// ```text
    /// /**
    ///   * Get the nsIFile for the file or directory.
    ///   *
    ///   * @return Returns the file currently selected
    ///   */
    /// ```
    ///

    /// `readonly attribute nsIFile file;`
    #[inline]
    pub unsafe fn GetFile(&self, aFile: *mut*const nsIFile) -> ::nserror::nsresult {
        ((*self.vtable).GetFile)(self, aFile)
    }


    /// ```text
    /// /**
    ///   * Get the nsIURI for the file or directory.
    ///   *
    ///   * @return Returns the file currently selected
    ///   */
    /// ```
    ///

    /// `readonly attribute nsIURI fileURL;`
    #[inline]
    pub unsafe fn GetFileURL(&self, aFileURL: *mut*const nsIURI) -> ::nserror::nsresult {
        ((*self.vtable).GetFileURL)(self, aFileURL)
    }


    /// ```text
    /// /**
    ///   * Get the enumerator for the selected files
    ///   * only works in the modeOpenMultiple mode
    ///   *
    ///   * @return Returns the files currently selected
    ///   */
    /// ```
    ///

    /// `readonly attribute nsISimpleEnumerator files;`
    #[inline]
    pub unsafe fn GetFiles(&self, aFiles: *mut*const nsISimpleEnumerator) -> ::nserror::nsresult {
        ((*self.vtable).GetFiles)(self, aFiles)
    }


    /// ```text
    /// /**
    ///   * Get the DOM File or the DOM Directory
    ///   *
    ///   * @return Returns the file or directory currently selected DOM object.
    ///   */
    /// ```
    ///

    /// `readonly attribute nsISupports domFileOrDirectory;`
    #[inline]
    pub unsafe fn GetDomFileOrDirectory(&self, aDomFileOrDirectory: *mut *const nsISupports) -> ::nserror::nsresult {
        ((*self.vtable).GetDomFileOrDirectory)(self, aDomFileOrDirectory)
    }


    /// ```text
    /// /**
    ///   * Get the enumerator for the selected files or directories
    ///   * only works in the modeOpenMultiple mode
    ///   *
    ///   * @return Returns the files/directories currently selected as DOM object.
    ///   */
    /// ```
    ///

    /// `readonly attribute nsISimpleEnumerator domFileOrDirectoryEnumerator;`
    #[inline]
    pub unsafe fn GetDomFileOrDirectoryEnumerator(&self, aDomFileOrDirectoryEnumerator: *mut*const nsISimpleEnumerator) -> ::nserror::nsresult {
        ((*self.vtable).GetDomFileOrDirectoryEnumerator)(self, aDomFileOrDirectoryEnumerator)
    }


    /// ```text
    /// /**
    ///   * Controls whether the chosen file(s) should be added to the system's recent
    ///   * documents list. This attribute will be ignored if the system has no "Recent
    ///   * Docs" concept, or if the application is in private browsing mode (in which
        ///   * case the file will not be added). Defaults to true.
    ///   */
    /// ```
    ///

    /// `attribute boolean addToRecentDocs;`
    #[inline]
    pub unsafe fn GetAddToRecentDocs(&self, aAddToRecentDocs: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetAddToRecentDocs)(self, aAddToRecentDocs)
    }


    /// ```text
    /// /**
    ///   * Controls whether the chosen file(s) should be added to the system's recent
    ///   * documents list. This attribute will be ignored if the system has no "Recent
    ///   * Docs" concept, or if the application is in private browsing mode (in which
        ///   * case the file will not be added). Defaults to true.
    ///   */
    /// ```
    ///

    /// `attribute boolean addToRecentDocs;`
    #[inline]
    pub unsafe fn SetAddToRecentDocs(&self, aAddToRecentDocs: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetAddToRecentDocs)(self, aAddToRecentDocs)
    }


    /// ```text
    /// /**
    ///   * Opens the file dialog asynchrounously.
    ///   * The passed in object's done method will be called upon completion.
    ///   */
    /// ```
    ///

    /// `void open (in nsIFilePickerShownCallback aFilePickerShownCallback);`
    #[inline]
    pub unsafe fn Open(&self, aFilePickerShownCallback: *const nsIFilePickerShownCallback) -> ::nserror::nsresult {
        ((*self.vtable).Open)(self, aFilePickerShownCallback)
    }


    /// ```text
    /// /**
    ///   * The picker's mode, as set by the 'mode' argument passed to init()
    ///   * (one of the modeOpen et. al. constants specified above).
    ///   */
    /// ```
    ///

    /// `readonly attribute short mode;`
    #[inline]
    pub unsafe fn GetMode(&self, aMode: *mut i16) -> ::nserror::nsresult {
        ((*self.vtable).GetMode)(self, aMode)
    }


    /// ```text
    /// /**
    ///    * If set to non-empty string, the nsIFilePicker implementation
    ///    * may use okButtonLabel as the label for the button the user uses to accept
    ///    * file selection.
    ///    */
    /// ```
    ///

    /// `attribute AString okButtonLabel;`
    #[inline]
    pub unsafe fn GetOkButtonLabel(&self, aOkButtonLabel: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetOkButtonLabel)(self, aOkButtonLabel)
    }


    /// ```text
    /// /**
    ///    * If set to non-empty string, the nsIFilePicker implementation
    ///    * may use okButtonLabel as the label for the button the user uses to accept
    ///    * file selection.
    ///    */
    /// ```
    ///

    /// `attribute AString okButtonLabel;`
    #[inline]
    pub unsafe fn SetOkButtonLabel(&self, aOkButtonLabel: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).SetOkButtonLabel)(self, aOkButtonLabel)
    }



    /// `attribute short capture;`
    #[inline]
    pub unsafe fn GetCapture(&self, aCapture: *mut i16) -> ::nserror::nsresult {
        ((*self.vtable).GetCapture)(self, aCapture)
    }



    /// `attribute short capture;`
    #[inline]
    pub unsafe fn SetCapture(&self, aCapture: i16) -> ::nserror::nsresult {
        ((*self.vtable).SetCapture)(self, aCapture)
    }


}


