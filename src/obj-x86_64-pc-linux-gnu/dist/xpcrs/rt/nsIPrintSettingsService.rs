//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/widget/nsIPrintSettingsService.idl
//


/// `interface nsIPrintSettingsService : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIPrintSettingsService {
    vtable: *const nsIPrintSettingsServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIPrintSettingsService.
unsafe impl XpCom for nsIPrintSettingsService {
    const IID: nsIID = nsID(0x841387c8, 0x72e6, 0x484b,
        [0x92, 0x96, 0xbf, 0x6e, 0xea, 0x80, 0xd5, 0x8a]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIPrintSettingsService {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIPrintSettingsService.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIPrintSettingsServiceCoerce {
    /// Cheaply cast a value of this type from a `nsIPrintSettingsService`.
    fn coerce_from(v: &nsIPrintSettingsService) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIPrintSettingsServiceCoerce for nsIPrintSettingsService {
    #[inline]
    fn coerce_from(v: &nsIPrintSettingsService) -> &Self {
        v
    }
}

impl nsIPrintSettingsService {
    /// Cast this `nsIPrintSettingsService` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIPrintSettingsServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIPrintSettingsService {
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
impl<T: nsISupportsCoerce> nsIPrintSettingsServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPrintSettingsService) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIPrintSettingsService
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIPrintSettingsServiceVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* [noscript] readonly attribute nsIPrintSettings defaultPrintSettingsForPrinting; */
    pub GetDefaultPrintSettingsForPrinting: unsafe extern "system" fn (this: *const nsIPrintSettingsService, aDefaultPrintSettingsForPrinting: *mut*const nsIPrintSettings) -> ::nserror::nsresult,

    /* readonly attribute nsIPrintSettings newPrintSettings; */
    pub GetNewPrintSettings: unsafe extern "system" fn (this: *const nsIPrintSettingsService, aNewPrintSettings: *mut*const nsIPrintSettings) -> ::nserror::nsresult,

    /* readonly attribute AString lastUsedPrinterName; */
    pub GetLastUsedPrinterName: unsafe extern "system" fn (this: *const nsIPrintSettingsService, aLastUsedPrinterName: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* void initPrintSettingsFromPrinter (in AString aPrinterName, in nsIPrintSettings aPrintSettings); */
    pub InitPrintSettingsFromPrinter: unsafe extern "system" fn (this: *const nsIPrintSettingsService, aPrinterName: *const ::nsstring::nsAString, aPrintSettings: *const nsIPrintSettings) -> ::nserror::nsresult,

    /* void initPrintSettingsFromPrefs (in nsIPrintSettings aPrintSettings, in boolean aUsePrinterNamePrefix, in unsigned long aFlags); */
    pub InitPrintSettingsFromPrefs: unsafe extern "system" fn (this: *const nsIPrintSettingsService, aPrintSettings: *const nsIPrintSettings, aUsePrinterNamePrefix: bool, aFlags: u32) -> ::nserror::nsresult,

    /* void savePrintSettingsToPrefs (in nsIPrintSettings aPrintSettings, in boolean aUsePrinterNamePrefix, in unsigned long aFlags); */
    pub SavePrintSettingsToPrefs: unsafe extern "system" fn (this: *const nsIPrintSettingsService, aPrintSettings: *const nsIPrintSettings, aUsePrinterNamePrefix: bool, aFlags: u32) -> ::nserror::nsresult,

    /* [noscript] void SerializeToPrintData (in nsIPrintSettings aPrintSettings, in PrintDataPtr data); */
    /// Unable to generate binding because `native type mozilla::embedding::PrintData unsupported`
    pub SerializeToPrintData: *const ::libc::c_void,

    /* [noscript] void DeserializeToPrintSettings (in PrintDataRef data, in nsIPrintSettings aPrintSettings); */
    /// Unable to generate binding because `native type const mozilla::embedding::PrintData unsupported`
    pub DeserializeToPrintSettings: *const ::libc::c_void,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIPrintSettingsService {

    /// ```text
    /// /**
    ///    * Returns the default print settings as used for printing.
    ///    */
    /// ```
    ///

    /// `[noscript] readonly attribute nsIPrintSettings defaultPrintSettingsForPrinting;`
    #[inline]
    pub unsafe fn GetDefaultPrintSettingsForPrinting(&self, aDefaultPrintSettingsForPrinting: *mut*const nsIPrintSettings) -> ::nserror::nsresult {
        ((*self.vtable).GetDefaultPrintSettingsForPrinting)(self, aDefaultPrintSettingsForPrinting)
    }


    /// ```text
    /// /**
    ///    * Returns a new, unique PrintSettings object each time.
    ///    *
    ///    * Initializes the newPrintSettings from the unprefixed printer
    ///    * (Note: this may not happen if there is an OS specific implementation.)
    ///    *
    ///    * XXX This should really be a function called `createPrintSettings()`.
    ///    */
    /// ```
    ///

    /// `readonly attribute nsIPrintSettings newPrintSettings;`
    #[inline]
    pub unsafe fn GetNewPrintSettings(&self, aNewPrintSettings: *mut*const nsIPrintSettings) -> ::nserror::nsresult {
        ((*self.vtable).GetNewPrintSettings)(self, aNewPrintSettings)
    }


    /// ```text
    /// /**
    ///    * The name of the last printer used. Note that this may not be set, or may
    ///    * no longer be a valid printer. The caller is responsible for checking and
    ///    * falling back to some other printer (such as the system default printer).
    ///    *
    ///    * XXX: make it [infallible] when AString supports that (bug 1491187).
    ///    */
    /// ```
    ///

    /// `readonly attribute AString lastUsedPrinterName;`
    #[inline]
    pub unsafe fn GetLastUsedPrinterName(&self, aLastUsedPrinterName: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetLastUsedPrinterName)(self, aLastUsedPrinterName)
    }


    /// ```text
    /// /**
    ///    * Initializes certain settings from the native printer into the PrintSettings
    ///    * if aPrinterName is null then it uses the default printer name if it can
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
    ///    * Reads PrintSettings values from Prefs,
    ///    * the values to be read are indicated by the "flags" arg.
    ///    *
    ///    * aPrintSettings should be initialized with the name of a printer. First
    ///    * it reads in the PrintSettings from the last print job. Then it uses the
    ///    * PrinterName in the PrinterSettings to read any settings that were saved
    ///    * just for that printer.
    ///    *
    ///    * aPS - PrintSettings to have its settings read
    ///    * aUsePrinterNamePrefix - indicates whether to use the printer name as a prefix
    ///    * aFlags - indicates which prefs to read, see nsIPrintSettings.idl for the
    ///    *          const values.
    ///    *
    ///    * Items not read:
    ///    *   startPageRange, endPageRange, scaling, printRange, title
    ///    *   docURL, isCancelled,
    ///    *   printSilent, shrinkToFit, numCopies,
    ///    *   printerName
    ///    *
    ///    */
    /// ```
    ///

    /// `void initPrintSettingsFromPrefs (in nsIPrintSettings aPrintSettings, in boolean aUsePrinterNamePrefix, in unsigned long aFlags);`
    #[inline]
    pub unsafe fn InitPrintSettingsFromPrefs(&self, aPrintSettings: *const nsIPrintSettings, aUsePrinterNamePrefix: bool, aFlags: u32) -> ::nserror::nsresult {
        ((*self.vtable).InitPrintSettingsFromPrefs)(self, aPrintSettings, aUsePrinterNamePrefix, aFlags)
    }


    /// ```text
    /// /**
    ///    * Writes PrintSettings values to Prefs,
    ///    * the values to be written are indicated by the "flags" arg.
    ///    *
    ///    * If there is no PrinterName in the PrinterSettings
    ///    * the values are saved as the "generic" values not associated with any printer.
    ///    * If a PrinterName is there, then it saves the items qualified for that Printer
    ///    *
    ///    * aPS - PrintSettings to have its settings saved
    ///    * aUsePrinterNamePrefix - indicates whether to use the printer name as a prefix
    ///    * aFlags - indicates which prefs to save, see nsIPrintSettings.idl for the const values.
    ///    *
    ///    * Items not written:
    ///    *   startPageRange, endPageRange, scaling, printRange, title
    ///    *   docURL, isCancelled,
    ///    *   printSilent, shrinkToFit, numCopies
    ///    *
    ///    */
    /// ```
    ///

    /// `void savePrintSettingsToPrefs (in nsIPrintSettings aPrintSettings, in boolean aUsePrinterNamePrefix, in unsigned long aFlags);`
    #[inline]
    pub unsafe fn SavePrintSettingsToPrefs(&self, aPrintSettings: *const nsIPrintSettings, aUsePrinterNamePrefix: bool, aFlags: u32) -> ::nserror::nsresult {
        ((*self.vtable).SavePrintSettingsToPrefs)(self, aPrintSettings, aUsePrinterNamePrefix, aFlags)
    }


    /// ```text
    /// /**
    ///    * Given some nsIPrintSettings,
    ///    * populates a PrintData representing them which can be sent over IPC. Values
    ///    * are only ever read from aSettings and aWBP.
    ///    *
    ///    * @param aSettings
    ///    *        An nsIPrintSettings for a print job.
    ///    * @param data
    ///    *        Pointer to a pre-existing PrintData to populate.
    ///    *
    ///    * @return nsresult
    ///    */
    /// ```
    ///

    /// `[noscript] void SerializeToPrintData (in nsIPrintSettings aPrintSettings, in PrintDataPtr data);`
    const _SerializeToPrintData: () = ();

    /// ```text
    /// /**
    ///    * This function is the opposite of SerializeToPrintData, in that it takes
    ///    * a PrintData, and populates a pre-existing nsIPrintSettings with the data
    ///    * from PrintData.
    ///    *
    ///    * @param PrintData
    ///    *        Printing information sent through IPC.
    ///    * @param settings
    ///    *        A pre-existing nsIPrintSettings to populate with the PrintData.
    ///    *
    ///    * @return nsresult
    ///    */
    /// ```
    ///

    /// `[noscript] void DeserializeToPrintSettings (in PrintDataRef data, in nsIPrintSettings aPrintSettings);`
    const _DeserializeToPrintSettings: () = ();

}


