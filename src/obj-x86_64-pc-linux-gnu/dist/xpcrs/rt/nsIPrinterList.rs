//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/widget/nsIPrinterList.idl
//


/// `interface nsIPrinterList : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIPrinterList {
    vtable: *const nsIPrinterListVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIPrinterList.
unsafe impl XpCom for nsIPrinterList {
    const IID: nsIID = nsID(0x5e738fff, 0x404c, 0x4c94,
        [0x91, 0x89, 0xe8, 0xf2, 0xcc, 0xe9, 0x3e, 0x94]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIPrinterList {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIPrinterList.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIPrinterListCoerce {
    /// Cheaply cast a value of this type from a `nsIPrinterList`.
    fn coerce_from(v: &nsIPrinterList) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIPrinterListCoerce for nsIPrinterList {
    #[inline]
    fn coerce_from(v: &nsIPrinterList) -> &Self {
        v
    }
}

impl nsIPrinterList {
    /// Cast this `nsIPrinterList` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIPrinterListCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIPrinterList {
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
impl<T: nsISupportsCoerce> nsIPrinterListCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPrinterList) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIPrinterList
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIPrinterListVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void initPrintSettingsFromPrinter (in AString aPrinterName, in nsIPrintSettings aPrintSettings); */
    pub InitPrintSettingsFromPrinter: unsafe extern "system" fn (this: *const nsIPrinterList, aPrinterName: *const ::nsstring::nsAString, aPrintSettings: *const nsIPrintSettings) -> ::nserror::nsresult,

    /* readonly attribute AString systemDefaultPrinterName; */
    pub GetSystemDefaultPrinterName: unsafe extern "system" fn (this: *const nsIPrinterList, aSystemDefaultPrinterName: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* [implicit_jscontext] Promise getPrinterByName (in AString aPrinterName); */
    /// Unable to generate binding because `jscontext is unsupported`
    pub GetPrinterByName: *const ::libc::c_void,

    /* [implicit_jscontext] Promise getPrinterBySystemName (in AString aPrinterName); */
    /// Unable to generate binding because `jscontext is unsupported`
    pub GetPrinterBySystemName: *const ::libc::c_void,

    /* [implicit_jscontext] Promise getNamedOrDefaultPrinter (in AString aPrinterName); */
    /// Unable to generate binding because `jscontext is unsupported`
    pub GetNamedOrDefaultPrinter: *const ::libc::c_void,

    /* [implicit_jscontext] readonly attribute Promise printers; */
    /// Unable to generate binding because `specialtype promise unsupported`
    pub GetPrinters: *const ::libc::c_void,

    /* [implicit_jscontext] readonly attribute Promise fallbackPaperList; */
    /// Unable to generate binding because `specialtype promise unsupported`
    pub GetFallbackPaperList: *const ::libc::c_void,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIPrinterList {

    /// ```text
    /// /**
    ///    * Initializes certain settings from the native printer into the PrintSettings
    ///    * These settings include, but are not limited to:
    ///    *   Page Orientation
    ///    *   Page Size
    ///    *   Number of Copies
    ///    */
    /// ```
    ///

    /// `void initPrintSettingsFromPrinter (in AString aPrinterName, in nsIPrintSettings aPrintSettings);`
    #[inline]
    pub unsafe fn InitPrintSettingsFromPrinter(&self, aPrinterName: *const ::nsstring::nsAString, aPrintSettings: *const nsIPrintSettings) -> ::nserror::nsresult {
        ((*self.vtable).InitPrintSettingsFromPrinter)(self, aPrinterName, aPrintSettings)
    }


    /// ```text
    /// /**
    ///    * The system default printer name. This is not necessarily gecko's
    ///    * default printer; see nsIPrintSettingsService.lastUsedPrinterName
    ///    * for that.
    ///    */
    /// ```
    ///

    /// `readonly attribute AString systemDefaultPrinterName;`
    #[inline]
    pub unsafe fn GetSystemDefaultPrinterName(&self, aSystemDefaultPrinterName: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetSystemDefaultPrinterName)(self, aSystemDefaultPrinterName)
    }


    /// ```text
    /// /**
    ///    * Returns a promise that resolves to the printer of a given name, or is
    ///    * rejected if there is no such printer.
    ///    */
    /// ```
    ///

    /// `[implicit_jscontext] Promise getPrinterByName (in AString aPrinterName);`
    const _GetPrinterByName: () = ();

    /// ```text
    /// /**
    ///    * Returns a promise that resolves to the printer of a given system name, or
    ///    * is rejected if there is no such printer.
    ///    * This may be more efficient than using getNamedPrinter, but requires the
    ///    * caller to know the system name of the printer they want to find.
    ///    */
    /// ```
    ///

    /// `[implicit_jscontext] Promise getPrinterBySystemName (in AString aPrinterName);`
    const _GetPrinterBySystemName: () = ();

    /// ```text
    /// /**
    ///    * Returns a promise that resolves to the printer of the given name, or
    ///    * the default system printer, or is rejected if there are no printers
    ///    * available.
    ///    */
    /// ```
    ///

    /// `[implicit_jscontext] Promise getNamedOrDefaultPrinter (in AString aPrinterName);`
    const _GetNamedOrDefaultPrinter: () = ();

    /// ```text
    /// /**
    ///    * Returns a promise that resolves to an array of printers.
    ///    */
    /// ```
    ///

    /// `[implicit_jscontext] readonly attribute Promise printers;`
    const _GetPrinters: () = ();

    /// ```text
    /// /**
    ///    * Returns a Promise that resolves to an array of nsIPaper instances
    ///    * for common paper sizes suitable to be supported for Save to PDF.
    ///    */
    /// ```
    ///

    /// `[implicit_jscontext] readonly attribute Promise fallbackPaperList;`
    const _GetFallbackPaperList: () = ();

}


