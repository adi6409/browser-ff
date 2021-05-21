//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/widget/nsIPrintSettings.idl
//


/// `interface nsIPrintSettings : nsISupports`
///

/// ```text
/// /**
///  * Simplified graphics interface for JS rendering.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIPrintSettings {
    vtable: *const nsIPrintSettingsVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIPrintSettings.
unsafe impl XpCom for nsIPrintSettings {
    const IID: nsIID = nsID(0xecc5cbad, 0x57fc, 0x4731,
        [0xb0, 0xbd, 0x09, 0xe8, 0x65, 0xbd, 0x62, 0xad]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIPrintSettings {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIPrintSettings.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIPrintSettingsCoerce {
    /// Cheaply cast a value of this type from a `nsIPrintSettings`.
    fn coerce_from(v: &nsIPrintSettings) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIPrintSettingsCoerce for nsIPrintSettings {
    #[inline]
    fn coerce_from(v: &nsIPrintSettings) -> &Self {
        v
    }
}

impl nsIPrintSettings {
    /// Cast this `nsIPrintSettings` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIPrintSettingsCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIPrintSettings {
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
impl<T: nsISupportsCoerce> nsIPrintSettingsCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPrintSettings) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIPrintSettings
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIPrintSettingsVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void GetEffectivePageSize (out double aWidth, out double aHeight); */
    pub GetEffectivePageSize: unsafe extern "system" fn (this: *const nsIPrintSettings, aWidth: *mut libc::c_double, aHeight: *mut libc::c_double) -> ::nserror::nsresult,

    /* [noscript,nostdcall,notxpcom] void GetEffectiveSheetSize (out double aWidth, out double aHeight); */
    /// Unable to generate binding because `nostdcall is unsupported`
    pub GetEffectiveSheetSize: *const ::libc::c_void,

    /* [noscript,nostdcall,notxpcom] long GetSheetOrientation (); */
    /// Unable to generate binding because `nostdcall is unsupported`
    pub GetSheetOrientation: *const ::libc::c_void,

    /* [noscript,nostdcall,notxpcom] bool HasOrthogonalSheetsAndPages (); */
    /// Unable to generate binding because `nostdcall is unsupported`
    pub HasOrthogonalSheetsAndPages: *const ::libc::c_void,

    /* nsIPrintSettings clone (); */
    pub Clone: unsafe extern "system" fn (this: *const nsIPrintSettings, _retval: *mut *const nsIPrintSettings) -> ::nserror::nsresult,

    /* void assign (in nsIPrintSettings aPS); */
    pub Assign: unsafe extern "system" fn (this: *const nsIPrintSettings, aPS: *const nsIPrintSettings) -> ::nserror::nsresult,

    /* [noscript] attribute nsIPrintSession printSession; */
    pub GetPrintSession: unsafe extern "system" fn (this: *const nsIPrintSettings, aPrintSession: *mut*const nsIPrintSession) -> ::nserror::nsresult,

    /* [noscript] attribute nsIPrintSession printSession; */
    pub SetPrintSession: unsafe extern "system" fn (this: *const nsIPrintSettings, aPrintSession: *const nsIPrintSession) -> ::nserror::nsresult,

    /* attribute double edgeTop; */
    pub GetEdgeTop: unsafe extern "system" fn (this: *const nsIPrintSettings, aEdgeTop: *mut libc::c_double) -> ::nserror::nsresult,

    /* attribute double edgeTop; */
    pub SetEdgeTop: unsafe extern "system" fn (this: *const nsIPrintSettings, aEdgeTop: libc::c_double) -> ::nserror::nsresult,

    /* attribute double edgeLeft; */
    pub GetEdgeLeft: unsafe extern "system" fn (this: *const nsIPrintSettings, aEdgeLeft: *mut libc::c_double) -> ::nserror::nsresult,

    /* attribute double edgeLeft; */
    pub SetEdgeLeft: unsafe extern "system" fn (this: *const nsIPrintSettings, aEdgeLeft: libc::c_double) -> ::nserror::nsresult,

    /* attribute double edgeBottom; */
    pub GetEdgeBottom: unsafe extern "system" fn (this: *const nsIPrintSettings, aEdgeBottom: *mut libc::c_double) -> ::nserror::nsresult,

    /* attribute double edgeBottom; */
    pub SetEdgeBottom: unsafe extern "system" fn (this: *const nsIPrintSettings, aEdgeBottom: libc::c_double) -> ::nserror::nsresult,

    /* attribute double edgeRight; */
    pub GetEdgeRight: unsafe extern "system" fn (this: *const nsIPrintSettings, aEdgeRight: *mut libc::c_double) -> ::nserror::nsresult,

    /* attribute double edgeRight; */
    pub SetEdgeRight: unsafe extern "system" fn (this: *const nsIPrintSettings, aEdgeRight: libc::c_double) -> ::nserror::nsresult,

    /* attribute double marginTop; */
    pub GetMarginTop: unsafe extern "system" fn (this: *const nsIPrintSettings, aMarginTop: *mut libc::c_double) -> ::nserror::nsresult,

    /* attribute double marginTop; */
    pub SetMarginTop: unsafe extern "system" fn (this: *const nsIPrintSettings, aMarginTop: libc::c_double) -> ::nserror::nsresult,

    /* attribute double marginLeft; */
    pub GetMarginLeft: unsafe extern "system" fn (this: *const nsIPrintSettings, aMarginLeft: *mut libc::c_double) -> ::nserror::nsresult,

    /* attribute double marginLeft; */
    pub SetMarginLeft: unsafe extern "system" fn (this: *const nsIPrintSettings, aMarginLeft: libc::c_double) -> ::nserror::nsresult,

    /* attribute double marginBottom; */
    pub GetMarginBottom: unsafe extern "system" fn (this: *const nsIPrintSettings, aMarginBottom: *mut libc::c_double) -> ::nserror::nsresult,

    /* attribute double marginBottom; */
    pub SetMarginBottom: unsafe extern "system" fn (this: *const nsIPrintSettings, aMarginBottom: libc::c_double) -> ::nserror::nsresult,

    /* attribute double marginRight; */
    pub GetMarginRight: unsafe extern "system" fn (this: *const nsIPrintSettings, aMarginRight: *mut libc::c_double) -> ::nserror::nsresult,

    /* attribute double marginRight; */
    pub SetMarginRight: unsafe extern "system" fn (this: *const nsIPrintSettings, aMarginRight: libc::c_double) -> ::nserror::nsresult,

    /* attribute double unwriteableMarginTop; */
    pub GetUnwriteableMarginTop: unsafe extern "system" fn (this: *const nsIPrintSettings, aUnwriteableMarginTop: *mut libc::c_double) -> ::nserror::nsresult,

    /* attribute double unwriteableMarginTop; */
    pub SetUnwriteableMarginTop: unsafe extern "system" fn (this: *const nsIPrintSettings, aUnwriteableMarginTop: libc::c_double) -> ::nserror::nsresult,

    /* attribute double unwriteableMarginLeft; */
    pub GetUnwriteableMarginLeft: unsafe extern "system" fn (this: *const nsIPrintSettings, aUnwriteableMarginLeft: *mut libc::c_double) -> ::nserror::nsresult,

    /* attribute double unwriteableMarginLeft; */
    pub SetUnwriteableMarginLeft: unsafe extern "system" fn (this: *const nsIPrintSettings, aUnwriteableMarginLeft: libc::c_double) -> ::nserror::nsresult,

    /* attribute double unwriteableMarginBottom; */
    pub GetUnwriteableMarginBottom: unsafe extern "system" fn (this: *const nsIPrintSettings, aUnwriteableMarginBottom: *mut libc::c_double) -> ::nserror::nsresult,

    /* attribute double unwriteableMarginBottom; */
    pub SetUnwriteableMarginBottom: unsafe extern "system" fn (this: *const nsIPrintSettings, aUnwriteableMarginBottom: libc::c_double) -> ::nserror::nsresult,

    /* attribute double unwriteableMarginRight; */
    pub GetUnwriteableMarginRight: unsafe extern "system" fn (this: *const nsIPrintSettings, aUnwriteableMarginRight: *mut libc::c_double) -> ::nserror::nsresult,

    /* attribute double unwriteableMarginRight; */
    pub SetUnwriteableMarginRight: unsafe extern "system" fn (this: *const nsIPrintSettings, aUnwriteableMarginRight: libc::c_double) -> ::nserror::nsresult,

    /* attribute double scaling; */
    pub GetScaling: unsafe extern "system" fn (this: *const nsIPrintSettings, aScaling: *mut libc::c_double) -> ::nserror::nsresult,

    /* attribute double scaling; */
    pub SetScaling: unsafe extern "system" fn (this: *const nsIPrintSettings, aScaling: libc::c_double) -> ::nserror::nsresult,

    /* [infallible] attribute boolean printBGColors; */
    pub GetPrintBGColors: unsafe extern "system" fn (this: *const nsIPrintSettings, aPrintBGColors: *mut bool) -> ::nserror::nsresult,

    /* [infallible] attribute boolean printBGColors; */
    pub SetPrintBGColors: unsafe extern "system" fn (this: *const nsIPrintSettings, aPrintBGColors: bool) -> ::nserror::nsresult,

    /* [infallible] attribute boolean printBGImages; */
    pub GetPrintBGImages: unsafe extern "system" fn (this: *const nsIPrintSettings, aPrintBGImages: *mut bool) -> ::nserror::nsresult,

    /* [infallible] attribute boolean printBGImages; */
    pub SetPrintBGImages: unsafe extern "system" fn (this: *const nsIPrintSettings, aPrintBGImages: bool) -> ::nserror::nsresult,

    /* [infallible] attribute boolean honorPageRuleMargins; */
    pub GetHonorPageRuleMargins: unsafe extern "system" fn (this: *const nsIPrintSettings, aHonorPageRuleMargins: *mut bool) -> ::nserror::nsresult,

    /* [infallible] attribute boolean honorPageRuleMargins; */
    pub SetHonorPageRuleMargins: unsafe extern "system" fn (this: *const nsIPrintSettings, aHonorPageRuleMargins: bool) -> ::nserror::nsresult,

    /* [infallible] attribute boolean showMarginGuides; */
    pub GetShowMarginGuides: unsafe extern "system" fn (this: *const nsIPrintSettings, aShowMarginGuides: *mut bool) -> ::nserror::nsresult,

    /* [infallible] attribute boolean showMarginGuides; */
    pub SetShowMarginGuides: unsafe extern "system" fn (this: *const nsIPrintSettings, aShowMarginGuides: bool) -> ::nserror::nsresult,

    /* [infallible] attribute boolean isPrintSelectionRBEnabled; */
    pub GetIsPrintSelectionRBEnabled: unsafe extern "system" fn (this: *const nsIPrintSettings, aIsPrintSelectionRBEnabled: *mut bool) -> ::nserror::nsresult,

    /* [infallible] attribute boolean isPrintSelectionRBEnabled; */
    pub SetIsPrintSelectionRBEnabled: unsafe extern "system" fn (this: *const nsIPrintSettings, aIsPrintSelectionRBEnabled: bool) -> ::nserror::nsresult,

    /* [infallible] attribute boolean printSelectionOnly; */
    pub GetPrintSelectionOnly: unsafe extern "system" fn (this: *const nsIPrintSettings, aPrintSelectionOnly: *mut bool) -> ::nserror::nsresult,

    /* [infallible] attribute boolean printSelectionOnly; */
    pub SetPrintSelectionOnly: unsafe extern "system" fn (this: *const nsIPrintSettings, aPrintSelectionOnly: bool) -> ::nserror::nsresult,

    /* attribute AString title; */
    pub GetTitle: unsafe extern "system" fn (this: *const nsIPrintSettings, aTitle: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* attribute AString title; */
    pub SetTitle: unsafe extern "system" fn (this: *const nsIPrintSettings, aTitle: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* attribute AString docURL; */
    pub GetDocURL: unsafe extern "system" fn (this: *const nsIPrintSettings, aDocURL: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* attribute AString docURL; */
    pub SetDocURL: unsafe extern "system" fn (this: *const nsIPrintSettings, aDocURL: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* attribute AString headerStrLeft; */
    pub GetHeaderStrLeft: unsafe extern "system" fn (this: *const nsIPrintSettings, aHeaderStrLeft: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* attribute AString headerStrLeft; */
    pub SetHeaderStrLeft: unsafe extern "system" fn (this: *const nsIPrintSettings, aHeaderStrLeft: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* attribute AString headerStrCenter; */
    pub GetHeaderStrCenter: unsafe extern "system" fn (this: *const nsIPrintSettings, aHeaderStrCenter: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* attribute AString headerStrCenter; */
    pub SetHeaderStrCenter: unsafe extern "system" fn (this: *const nsIPrintSettings, aHeaderStrCenter: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* attribute AString headerStrRight; */
    pub GetHeaderStrRight: unsafe extern "system" fn (this: *const nsIPrintSettings, aHeaderStrRight: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* attribute AString headerStrRight; */
    pub SetHeaderStrRight: unsafe extern "system" fn (this: *const nsIPrintSettings, aHeaderStrRight: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* attribute AString footerStrLeft; */
    pub GetFooterStrLeft: unsafe extern "system" fn (this: *const nsIPrintSettings, aFooterStrLeft: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* attribute AString footerStrLeft; */
    pub SetFooterStrLeft: unsafe extern "system" fn (this: *const nsIPrintSettings, aFooterStrLeft: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* attribute AString footerStrCenter; */
    pub GetFooterStrCenter: unsafe extern "system" fn (this: *const nsIPrintSettings, aFooterStrCenter: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* attribute AString footerStrCenter; */
    pub SetFooterStrCenter: unsafe extern "system" fn (this: *const nsIPrintSettings, aFooterStrCenter: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* attribute AString footerStrRight; */
    pub GetFooterStrRight: unsafe extern "system" fn (this: *const nsIPrintSettings, aFooterStrRight: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* attribute AString footerStrRight; */
    pub SetFooterStrRight: unsafe extern "system" fn (this: *const nsIPrintSettings, aFooterStrRight: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* attribute boolean isCancelled; */
    pub GetIsCancelled: unsafe extern "system" fn (this: *const nsIPrintSettings, aIsCancelled: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean isCancelled; */
    pub SetIsCancelled: unsafe extern "system" fn (this: *const nsIPrintSettings, aIsCancelled: bool) -> ::nserror::nsresult,

    /* readonly attribute boolean saveOnCancel; */
    pub GetSaveOnCancel: unsafe extern "system" fn (this: *const nsIPrintSettings, aSaveOnCancel: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean printSilent; */
    pub GetPrintSilent: unsafe extern "system" fn (this: *const nsIPrintSettings, aPrintSilent: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean printSilent; */
    pub SetPrintSilent: unsafe extern "system" fn (this: *const nsIPrintSettings, aPrintSilent: bool) -> ::nserror::nsresult,

    /* attribute boolean shrinkToFit; */
    pub GetShrinkToFit: unsafe extern "system" fn (this: *const nsIPrintSettings, aShrinkToFit: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean shrinkToFit; */
    pub SetShrinkToFit: unsafe extern "system" fn (this: *const nsIPrintSettings, aShrinkToFit: bool) -> ::nserror::nsresult,

    /* attribute boolean showPrintProgress; */
    pub GetShowPrintProgress: unsafe extern "system" fn (this: *const nsIPrintSettings, aShowPrintProgress: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean showPrintProgress; */
    pub SetShowPrintProgress: unsafe extern "system" fn (this: *const nsIPrintSettings, aShowPrintProgress: bool) -> ::nserror::nsresult,

    /* attribute AString paperId; */
    pub GetPaperId: unsafe extern "system" fn (this: *const nsIPrintSettings, aPaperId: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* attribute AString paperId; */
    pub SetPaperId: unsafe extern "system" fn (this: *const nsIPrintSettings, aPaperId: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* attribute double paperWidth; */
    pub GetPaperWidth: unsafe extern "system" fn (this: *const nsIPrintSettings, aPaperWidth: *mut libc::c_double) -> ::nserror::nsresult,

    /* attribute double paperWidth; */
    pub SetPaperWidth: unsafe extern "system" fn (this: *const nsIPrintSettings, aPaperWidth: libc::c_double) -> ::nserror::nsresult,

    /* attribute double paperHeight; */
    pub GetPaperHeight: unsafe extern "system" fn (this: *const nsIPrintSettings, aPaperHeight: *mut libc::c_double) -> ::nserror::nsresult,

    /* attribute double paperHeight; */
    pub SetPaperHeight: unsafe extern "system" fn (this: *const nsIPrintSettings, aPaperHeight: libc::c_double) -> ::nserror::nsresult,

    /* attribute short paperSizeUnit; */
    pub GetPaperSizeUnit: unsafe extern "system" fn (this: *const nsIPrintSettings, aPaperSizeUnit: *mut i16) -> ::nserror::nsresult,

    /* attribute short paperSizeUnit; */
    pub SetPaperSizeUnit: unsafe extern "system" fn (this: *const nsIPrintSettings, aPaperSizeUnit: i16) -> ::nserror::nsresult,

    /* attribute boolean printReversed; */
    pub GetPrintReversed: unsafe extern "system" fn (this: *const nsIPrintSettings, aPrintReversed: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean printReversed; */
    pub SetPrintReversed: unsafe extern "system" fn (this: *const nsIPrintSettings, aPrintReversed: bool) -> ::nserror::nsresult,

    /* [infallible] attribute boolean printInColor; */
    pub GetPrintInColor: unsafe extern "system" fn (this: *const nsIPrintSettings, aPrintInColor: *mut bool) -> ::nserror::nsresult,

    /* [infallible] attribute boolean printInColor; */
    pub SetPrintInColor: unsafe extern "system" fn (this: *const nsIPrintSettings, aPrintInColor: bool) -> ::nserror::nsresult,

    /* attribute long orientation; */
    pub GetOrientation: unsafe extern "system" fn (this: *const nsIPrintSettings, aOrientation: *mut i32) -> ::nserror::nsresult,

    /* attribute long orientation; */
    pub SetOrientation: unsafe extern "system" fn (this: *const nsIPrintSettings, aOrientation: i32) -> ::nserror::nsresult,

    /* attribute long numCopies; */
    pub GetNumCopies: unsafe extern "system" fn (this: *const nsIPrintSettings, aNumCopies: *mut i32) -> ::nserror::nsresult,

    /* attribute long numCopies; */
    pub SetNumCopies: unsafe extern "system" fn (this: *const nsIPrintSettings, aNumCopies: i32) -> ::nserror::nsresult,

    /* attribute long numPagesPerSheet; */
    pub GetNumPagesPerSheet: unsafe extern "system" fn (this: *const nsIPrintSettings, aNumPagesPerSheet: *mut i32) -> ::nserror::nsresult,

    /* attribute long numPagesPerSheet; */
    pub SetNumPagesPerSheet: unsafe extern "system" fn (this: *const nsIPrintSettings, aNumPagesPerSheet: i32) -> ::nserror::nsresult,

    /* attribute AString printerName; */
    pub GetPrinterName: unsafe extern "system" fn (this: *const nsIPrintSettings, aPrinterName: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* attribute AString printerName; */
    pub SetPrinterName: unsafe extern "system" fn (this: *const nsIPrintSettings, aPrinterName: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* attribute boolean printToFile; */
    pub GetPrintToFile: unsafe extern "system" fn (this: *const nsIPrintSettings, aPrintToFile: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean printToFile; */
    pub SetPrintToFile: unsafe extern "system" fn (this: *const nsIPrintSettings, aPrintToFile: bool) -> ::nserror::nsresult,

    /* attribute AString toFileName; */
    pub GetToFileName: unsafe extern "system" fn (this: *const nsIPrintSettings, aToFileName: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* attribute AString toFileName; */
    pub SetToFileName: unsafe extern "system" fn (this: *const nsIPrintSettings, aToFileName: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* attribute short outputFormat; */
    pub GetOutputFormat: unsafe extern "system" fn (this: *const nsIPrintSettings, aOutputFormat: *mut i16) -> ::nserror::nsresult,

    /* attribute short outputFormat; */
    pub SetOutputFormat: unsafe extern "system" fn (this: *const nsIPrintSettings, aOutputFormat: i16) -> ::nserror::nsresult,

    /* attribute long printPageDelay; */
    pub GetPrintPageDelay: unsafe extern "system" fn (this: *const nsIPrintSettings, aPrintPageDelay: *mut i32) -> ::nserror::nsresult,

    /* attribute long printPageDelay; */
    pub SetPrintPageDelay: unsafe extern "system" fn (this: *const nsIPrintSettings, aPrintPageDelay: i32) -> ::nserror::nsresult,

    /* attribute long resolution; */
    pub GetResolution: unsafe extern "system" fn (this: *const nsIPrintSettings, aResolution: *mut i32) -> ::nserror::nsresult,

    /* attribute long resolution; */
    pub SetResolution: unsafe extern "system" fn (this: *const nsIPrintSettings, aResolution: i32) -> ::nserror::nsresult,

    /* attribute long duplex; */
    pub GetDuplex: unsafe extern "system" fn (this: *const nsIPrintSettings, aDuplex: *mut i32) -> ::nserror::nsresult,

    /* attribute long duplex; */
    pub SetDuplex: unsafe extern "system" fn (this: *const nsIPrintSettings, aDuplex: i32) -> ::nserror::nsresult,

    /* attribute boolean isInitializedFromPrinter; */
    pub GetIsInitializedFromPrinter: unsafe extern "system" fn (this: *const nsIPrintSettings, aIsInitializedFromPrinter: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean isInitializedFromPrinter; */
    pub SetIsInitializedFromPrinter: unsafe extern "system" fn (this: *const nsIPrintSettings, aIsInitializedFromPrinter: bool) -> ::nserror::nsresult,

    /* attribute boolean isInitializedFromPrefs; */
    pub GetIsInitializedFromPrefs: unsafe extern "system" fn (this: *const nsIPrintSettings, aIsInitializedFromPrefs: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean isInitializedFromPrefs; */
    pub SetIsInitializedFromPrefs: unsafe extern "system" fn (this: *const nsIPrintSettings, aIsInitializedFromPrefs: bool) -> ::nserror::nsresult,

    /* [noscript] void SetMarginInTwips (in nsNativeIntMarginRef aMargin); */
    /// Unable to generate binding because `native type nsIntMargin unsupported`
    pub SetMarginInTwips: *const ::libc::c_void,

    /* [noscript] void SetEdgeInTwips (in nsNativeIntMarginRef aEdge); */
    /// Unable to generate binding because `native type nsIntMargin unsupported`
    pub SetEdgeInTwips: *const ::libc::c_void,

    /* [noscript,nostdcall,notxpcom] nsNativeIntMargin GetMarginInTwips (); */
    /// Unable to generate binding because `nostdcall is unsupported`
    pub GetMarginInTwips: *const ::libc::c_void,

    /* [noscript,nostdcall,notxpcom] nsNativeIntMargin GetEdgeInTwips (); */
    /// Unable to generate binding because `nostdcall is unsupported`
    pub GetEdgeInTwips: *const ::libc::c_void,

    /* [noscript] void SetupSilentPrinting (); */
    pub SetupSilentPrinting: unsafe extern "system" fn (this: *const nsIPrintSettings) -> ::nserror::nsresult,

    /* [noscript] void SetUnwriteableMarginInTwips (in nsNativeIntMarginRef aEdge); */
    /// Unable to generate binding because `native type nsIntMargin unsupported`
    pub SetUnwriteableMarginInTwips: *const ::libc::c_void,

    /* [noscript,nostdcall,notxpcom] nsNativeIntMargin GetUnwriteableMarginInTwips (); */
    /// Unable to generate binding because `nostdcall is unsupported`
    pub GetUnwriteableMarginInTwips: *const ::libc::c_void,

    /* attribute Array<long> pageRanges; */
    pub GetPageRanges: unsafe extern "system" fn (this: *const nsIPrintSettings, aPageRanges: *mut thin_vec::ThinVec<i32>) -> ::nserror::nsresult,

    /* attribute Array<long> pageRanges; */
    pub SetPageRanges: unsafe extern "system" fn (this: *const nsIPrintSettings, aPageRanges: *const thin_vec::ThinVec<i32>) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIPrintSettings {
    /// ```text
    /// /**
    ///    * PrintSettings to be Saved Navigation Constants
    ///    */
    /// ```
    ///

    pub const kInitSaveHeaderLeft: i64 = 2;


    pub const kInitSaveHeaderCenter: i64 = 4;


    pub const kInitSaveHeaderRight: i64 = 8;


    pub const kInitSaveFooterLeft: i64 = 16;


    pub const kInitSaveFooterCenter: i64 = 32;


    pub const kInitSaveFooterRight: i64 = 64;


    pub const kInitSaveBGColors: i64 = 128;


    pub const kInitSaveBGImages: i64 = 256;


    pub const kInitSavePaperSize: i64 = 512;


    pub const kInitSaveResolution: i64 = 1024;


    pub const kInitSaveDuplex: i64 = 2048;


    pub const kInitSaveUnwriteableMargins: i64 = 16384;


    pub const kInitSaveEdges: i64 = 32768;


    pub const kInitSaveReversed: i64 = 65536;


    pub const kInitSaveInColor: i64 = 131072;


    pub const kInitSaveOrientation: i64 = 262144;


    pub const kInitSavePrinterName: i64 = 1048576;


    pub const kInitSavePrintToFile: i64 = 2097152;


    pub const kInitSaveToFileName: i64 = 4194304;


    pub const kInitSavePageDelay: i64 = 8388608;


    pub const kInitSaveMargins: i64 = 16777216;


    pub const kInitSaveNativeData: i64 = 33554432;


    pub const kInitSaveShrinkToFit: i64 = 134217728;


    pub const kInitSaveScaling: i64 = 268435456;


    pub const kInitSaveAll: i64 = 4294967295;


    pub const kGlobalSettings: i64 = 134447614;


    pub const kJustLeft: i64 = 0;


    pub const kJustCenter: i64 = 1;


    pub const kJustRight: i64 = 2;

    /// ```text
    /// /**
    ///    * Page Size Unit Constants
    ///    */
    /// ```
    ///

    pub const kPaperSizeInches: i64 = 0;


    pub const kPaperSizeMillimeters: i64 = 1;

    /// ```text
    /// /**
    ///    * Orientation Constants
    ///    */
    /// ```
    ///

    pub const kPortraitOrientation: i64 = 0;


    pub const kLandscapeOrientation: i64 = 1;

    /// ```text
    /// /**
    ///    * Output file format
    ///    */
    /// ```
    ///

    pub const kOutputFormatNative: i64 = 0;


    pub const kOutputFormatPS: i64 = 1;


    pub const kOutputFormatPDF: i64 = 2;

    /// ```text
    /// /**
    ///    * Duplex printing options.
    ///    *
    ///    * Note that other libraries refer to equivalent duplex settings using
    ///    * various sets of terminology. This can be confusing and inconsistent both
    ///    * with other libraries, and with the behavior that these terms intend to describe.
    ///    *
    ///    * kDuplexNone is equivalent to Simplex. Thankfully, both of these terms are
    ///    * consistent with the behavior that they describe, which is to have single-sided
    ///    * printing per sheet.
    ///    *
    ///    * kDuplexFlipOnLongEdge is equivalent to the following platform-specific constants:
    ///    *   CUPS/macOS: NoTumble
    ///    *      Windows: DMDUP_VERTICAL
    ///    *          GTK: GTK_PRINT_DUPLEX_HORIZONTAL
    ///    *
    ///    * kDuplexFlipOnShortEdge is equivalent to the following platform-specific constants:
    ///    *   CUPS/macOS: Tumble
    ///    *      Windows: DMDUP_HORIZONTAL
    ///    *          GTK: GTK_PRINT_DUPLEX_VERTICAL
    ///    *
    ///    *
    ///    * Notice that the GTK and Windows constants have opposite meanings for
    ///    * VERTICAL and HORIZONTAL.
    ///    *
    ///    * To make matters more confusing, these platform-specific terms describe different
    ///    * behavior (from the user's perspective) depending on whether the sheet is in
    ///    * portrait vs. landscape orientation.
    ///    *
    ///    * For example, the generic term "tumble" describes behavior where a sheet flips over
    ///    * a binding on the top edge (like a calendar). This requires that the back side of
    ///    * the sheet is printed upside down with respect to the front side of the sheet,
    ///    * so that its content appears upright to the reader when they tumble-flip the
    ///    * sheet over the top-edge binding.
    ///    *
    ///    * However, the CUPS/macOS Tumble setting only inverts the back side of the
    ///    * sheet in portrait orientation. When you switch to landscape orientation, the
    ///    * Tumble setting behaves like a book-like sheet flip, where the front and back
    ///    * sides of the sheet are both printed upright with respect to each other.
    ///    *
    ///    * This is why it is more consistent and more clear to think of these terms
    ///    * with regard to sheets being bound on the long edge or the short edge.
    ///    *
    ///    * kDuplexFlipOnLongEdge  + Portrait  =     book-like flip (front/back same direction)
    ///    * kDuplexFlipOnLongEdge  + Landscape = calendar-like flip (front/back inverted)
    ///    *
    ///    * kDuplexFlipOnShortEdge + Portrait  = calendar-like flip (front/back inverted)
    ///    * kDuplexFlipOnShortEdge + Landscape =     book-like flip (front/back same direction)
    ///    *
    ///    * The long-edge and short-edge terminology unfortunately breaks down when printing
    ///    * with square sheet dimensions. Thankfully this edge case (hah) is quite uncommon,
    ///    * since most standard printing paper dimensions are not square. Such a paper size
    ///    * would even break the uniformly used portrait and landscape terminology.
    ///    */
    /// ```
    ///

    pub const kDuplexNone: i64 = 0;


    pub const kDuplexFlipOnLongEdge: i64 = 1;


    pub const kDuplexFlipOnShortEdge: i64 = 2;

    /// ```text
    /// /**
    ///    * Get the page size in twips, considering the
    ///    * orientation (portrait or landscape).
    ///    */
    /// ```
    ///

    /// `void GetEffectivePageSize (out double aWidth, out double aHeight);`
    #[inline]
    pub unsafe fn GetEffectivePageSize(&self, aWidth: *mut libc::c_double, aHeight: *mut libc::c_double) -> ::nserror::nsresult {
        ((*self.vtable).GetEffectivePageSize)(self, aWidth, aHeight)
    }


    /// ```text
    /// /**
    ///    * Get the printed sheet size in twips, considering both the user-specified
    ///    * orientation (portrait or landscape) *as well as* the fact that we might be
    ///    * inverting the orientation to account for 2 or 6 pages-per-sheet.
    ///    *
    ///    * This API will usually behave the same (& return the same thing) as
    ///    * GetEffectivePageSize, *except for* when we are printing with 2 or 6
    ///    * pages-per-sheet, in which case the return values (aWidth & aHeight) will
    ///    * be swapped with respect to what GetEffectivePageSize would return.
    ///    *
    ///    * Callers should use this method rather than GetEffectivePageSize when they
    ///    * really do want the size of the sheet of paper to be printed, rather than
    ///    * the possibly-"virtualized"-via-pages-per-sheet page size.
    ///    */
    /// ```
    ///

    /// `[noscript,nostdcall,notxpcom] void GetEffectiveSheetSize (out double aWidth, out double aHeight);`
    const _GetEffectiveSheetSize: () = ();

    /// ```text
    /// /**
    ///    * Get the orientation of a printed sheet. This is usually the same as the
    ///    * 'orientation' attribute (which is the orientation of individual pages),
    ///    * except when we're printing with 2 or 6 pages-per-sheet, in which case
    ///    * it'll be the opposite value.
    ///    *
    ///    * Note that this value is not independently settable. Its value is fully
    ///    * determined by the 'orientation' and 'numPagesPerSheet' attributes.
    ///    */
    /// ```
    ///

    /// `[noscript,nostdcall,notxpcom] long GetSheetOrientation ();`
    const _GetSheetOrientation: () = ();

    /// ```text
    /// /**
    ///    * Convenience getter, which returns true IFF the sheet orientation and the
    ///    * page orientation are orthogonal. (In other words, returns true IFF we
        ///    * are printing with 2 or 6 pages-per-sheet.)
    ///    */
    /// ```
    ///

    /// `[noscript,nostdcall,notxpcom] bool HasOrthogonalSheetsAndPages ();`
    const _HasOrthogonalSheetsAndPages: () = ();

    /// ```text
    /// /**
    ///    * Makes a new copy
    ///    */
    /// ```
    ///

    /// `nsIPrintSettings clone ();`
    #[inline]
    pub unsafe fn Clone(&self, _retval: *mut *const nsIPrintSettings) -> ::nserror::nsresult {
        ((*self.vtable).Clone)(self, _retval)
    }


    /// ```text
    /// /**
    ///    * Assigns the internal values from the "in" arg to the current object
    ///    */
    /// ```
    ///

    /// `void assign (in nsIPrintSettings aPS);`
    #[inline]
    pub unsafe fn Assign(&self, aPS: *const nsIPrintSettings) -> ::nserror::nsresult {
        ((*self.vtable).Assign)(self, aPS)
    }


    /// ```text
    /// /**
    ///    * Data Members
    ///    */
    /// ```
    ///

    /// `[noscript] attribute nsIPrintSession printSession;`
    #[inline]
    pub unsafe fn GetPrintSession(&self, aPrintSession: *mut*const nsIPrintSession) -> ::nserror::nsresult {
        ((*self.vtable).GetPrintSession)(self, aPrintSession)
    }


    /// ```text
    /// /**
    ///    * Data Members
    ///    */
    /// ```
    ///

    /// `[noscript] attribute nsIPrintSession printSession;`
    #[inline]
    pub unsafe fn SetPrintSession(&self, aPrintSession: *const nsIPrintSession) -> ::nserror::nsresult {
        ((*self.vtable).SetPrintSession)(self, aPrintSession)
    }


    /// ```text
    /// /**
    ///    * The edge measurements define the positioning of the headers
    ///    * and footers on the page. They're treated as an offset from the edges of
    ///    * the page, but are forced to be at least the "unwriteable margin"
    ///    * (described below).
    ///    */
    /// ```
    ///

    /// `attribute double edgeTop;`
    #[inline]
    pub unsafe fn GetEdgeTop(&self, aEdgeTop: *mut libc::c_double) -> ::nserror::nsresult {
        ((*self.vtable).GetEdgeTop)(self, aEdgeTop)
    }


    /// ```text
    /// /**
    ///    * The edge measurements define the positioning of the headers
    ///    * and footers on the page. They're treated as an offset from the edges of
    ///    * the page, but are forced to be at least the "unwriteable margin"
    ///    * (described below).
    ///    */
    /// ```
    ///

    /// `attribute double edgeTop;`
    #[inline]
    pub unsafe fn SetEdgeTop(&self, aEdgeTop: libc::c_double) -> ::nserror::nsresult {
        ((*self.vtable).SetEdgeTop)(self, aEdgeTop)
    }



    /// `attribute double edgeLeft;`
    #[inline]
    pub unsafe fn GetEdgeLeft(&self, aEdgeLeft: *mut libc::c_double) -> ::nserror::nsresult {
        ((*self.vtable).GetEdgeLeft)(self, aEdgeLeft)
    }



    /// `attribute double edgeLeft;`
    #[inline]
    pub unsafe fn SetEdgeLeft(&self, aEdgeLeft: libc::c_double) -> ::nserror::nsresult {
        ((*self.vtable).SetEdgeLeft)(self, aEdgeLeft)
    }



    /// `attribute double edgeBottom;`
    #[inline]
    pub unsafe fn GetEdgeBottom(&self, aEdgeBottom: *mut libc::c_double) -> ::nserror::nsresult {
        ((*self.vtable).GetEdgeBottom)(self, aEdgeBottom)
    }



    /// `attribute double edgeBottom;`
    #[inline]
    pub unsafe fn SetEdgeBottom(&self, aEdgeBottom: libc::c_double) -> ::nserror::nsresult {
        ((*self.vtable).SetEdgeBottom)(self, aEdgeBottom)
    }



    /// `attribute double edgeRight;`
    #[inline]
    pub unsafe fn GetEdgeRight(&self, aEdgeRight: *mut libc::c_double) -> ::nserror::nsresult {
        ((*self.vtable).GetEdgeRight)(self, aEdgeRight)
    }



    /// `attribute double edgeRight;`
    #[inline]
    pub unsafe fn SetEdgeRight(&self, aEdgeRight: libc::c_double) -> ::nserror::nsresult {
        ((*self.vtable).SetEdgeRight)(self, aEdgeRight)
    }


    /// ```text
    /// /**
    ///    * The margins define the positioning of the content on the page.
    ///    * and footers on the page. They're treated as an offset from the edges of
    ///    * the page, but are forced to be at least the "unwriteable margin"
    ///    * (described below).
    ///    */
    /// ```
    ///

    /// `attribute double marginTop;`
    #[inline]
    pub unsafe fn GetMarginTop(&self, aMarginTop: *mut libc::c_double) -> ::nserror::nsresult {
        ((*self.vtable).GetMarginTop)(self, aMarginTop)
    }


    /// ```text
    /// /**
    ///    * The margins define the positioning of the content on the page.
    ///    * and footers on the page. They're treated as an offset from the edges of
    ///    * the page, but are forced to be at least the "unwriteable margin"
    ///    * (described below).
    ///    */
    /// ```
    ///

    /// `attribute double marginTop;`
    #[inline]
    pub unsafe fn SetMarginTop(&self, aMarginTop: libc::c_double) -> ::nserror::nsresult {
        ((*self.vtable).SetMarginTop)(self, aMarginTop)
    }



    /// `attribute double marginLeft;`
    #[inline]
    pub unsafe fn GetMarginLeft(&self, aMarginLeft: *mut libc::c_double) -> ::nserror::nsresult {
        ((*self.vtable).GetMarginLeft)(self, aMarginLeft)
    }



    /// `attribute double marginLeft;`
    #[inline]
    pub unsafe fn SetMarginLeft(&self, aMarginLeft: libc::c_double) -> ::nserror::nsresult {
        ((*self.vtable).SetMarginLeft)(self, aMarginLeft)
    }



    /// `attribute double marginBottom;`
    #[inline]
    pub unsafe fn GetMarginBottom(&self, aMarginBottom: *mut libc::c_double) -> ::nserror::nsresult {
        ((*self.vtable).GetMarginBottom)(self, aMarginBottom)
    }



    /// `attribute double marginBottom;`
    #[inline]
    pub unsafe fn SetMarginBottom(&self, aMarginBottom: libc::c_double) -> ::nserror::nsresult {
        ((*self.vtable).SetMarginBottom)(self, aMarginBottom)
    }



    /// `attribute double marginRight;`
    #[inline]
    pub unsafe fn GetMarginRight(&self, aMarginRight: *mut libc::c_double) -> ::nserror::nsresult {
        ((*self.vtable).GetMarginRight)(self, aMarginRight)
    }



    /// `attribute double marginRight;`
    #[inline]
    pub unsafe fn SetMarginRight(&self, aMarginRight: libc::c_double) -> ::nserror::nsresult {
        ((*self.vtable).SetMarginRight)(self, aMarginRight)
    }


    /// ```text
    /// /**
    ///    * The unwriteable margin defines the printable region of the paper.
    ///    */
    /// ```
    ///

    /// `attribute double unwriteableMarginTop;`
    #[inline]
    pub unsafe fn GetUnwriteableMarginTop(&self, aUnwriteableMarginTop: *mut libc::c_double) -> ::nserror::nsresult {
        ((*self.vtable).GetUnwriteableMarginTop)(self, aUnwriteableMarginTop)
    }


    /// ```text
    /// /**
    ///    * The unwriteable margin defines the printable region of the paper.
    ///    */
    /// ```
    ///

    /// `attribute double unwriteableMarginTop;`
    #[inline]
    pub unsafe fn SetUnwriteableMarginTop(&self, aUnwriteableMarginTop: libc::c_double) -> ::nserror::nsresult {
        ((*self.vtable).SetUnwriteableMarginTop)(self, aUnwriteableMarginTop)
    }



    /// `attribute double unwriteableMarginLeft;`
    #[inline]
    pub unsafe fn GetUnwriteableMarginLeft(&self, aUnwriteableMarginLeft: *mut libc::c_double) -> ::nserror::nsresult {
        ((*self.vtable).GetUnwriteableMarginLeft)(self, aUnwriteableMarginLeft)
    }



    /// `attribute double unwriteableMarginLeft;`
    #[inline]
    pub unsafe fn SetUnwriteableMarginLeft(&self, aUnwriteableMarginLeft: libc::c_double) -> ::nserror::nsresult {
        ((*self.vtable).SetUnwriteableMarginLeft)(self, aUnwriteableMarginLeft)
    }



    /// `attribute double unwriteableMarginBottom;`
    #[inline]
    pub unsafe fn GetUnwriteableMarginBottom(&self, aUnwriteableMarginBottom: *mut libc::c_double) -> ::nserror::nsresult {
        ((*self.vtable).GetUnwriteableMarginBottom)(self, aUnwriteableMarginBottom)
    }



    /// `attribute double unwriteableMarginBottom;`
    #[inline]
    pub unsafe fn SetUnwriteableMarginBottom(&self, aUnwriteableMarginBottom: libc::c_double) -> ::nserror::nsresult {
        ((*self.vtable).SetUnwriteableMarginBottom)(self, aUnwriteableMarginBottom)
    }



    /// `attribute double unwriteableMarginRight;`
    #[inline]
    pub unsafe fn GetUnwriteableMarginRight(&self, aUnwriteableMarginRight: *mut libc::c_double) -> ::nserror::nsresult {
        ((*self.vtable).GetUnwriteableMarginRight)(self, aUnwriteableMarginRight)
    }



    /// `attribute double unwriteableMarginRight;`
    #[inline]
    pub unsafe fn SetUnwriteableMarginRight(&self, aUnwriteableMarginRight: libc::c_double) -> ::nserror::nsresult {
        ((*self.vtable).SetUnwriteableMarginRight)(self, aUnwriteableMarginRight)
    }



    /// `attribute double scaling;`
    #[inline]
    pub unsafe fn GetScaling(&self, aScaling: *mut libc::c_double) -> ::nserror::nsresult {
        ((*self.vtable).GetScaling)(self, aScaling)
    }



    /// `attribute double scaling;`
    #[inline]
    pub unsafe fn SetScaling(&self, aScaling: libc::c_double) -> ::nserror::nsresult {
        ((*self.vtable).SetScaling)(self, aScaling)
    }



    /// `[infallible] attribute boolean printBGColors;`
    #[inline]
    pub unsafe fn GetPrintBGColors(&self) -> bool {
        let mut result = <bool as ::std::default::Default>::default();
        let _rv = ((*self.vtable).GetPrintBGColors)(self, &mut result);
        debug_assert!(_rv.succeeded());
        result
    }



    /// `[infallible] attribute boolean printBGColors;`
    #[inline]
    pub unsafe fn SetPrintBGColors(&self, aPrintBGColors: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetPrintBGColors)(self, aPrintBGColors)
    }



    /// `[infallible] attribute boolean printBGImages;`
    #[inline]
    pub unsafe fn GetPrintBGImages(&self) -> bool {
        let mut result = <bool as ::std::default::Default>::default();
        let _rv = ((*self.vtable).GetPrintBGImages)(self, &mut result);
        debug_assert!(_rv.succeeded());
        result
    }



    /// `[infallible] attribute boolean printBGImages;`
    #[inline]
    pub unsafe fn SetPrintBGImages(&self, aPrintBGImages: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetPrintBGImages)(self, aPrintBGImages)
    }


    /// ```text
    /// /** Whether @page rule margins should be honored or not. */
    /// ```
    ///

    /// `[infallible] attribute boolean honorPageRuleMargins;`
    #[inline]
    pub unsafe fn GetHonorPageRuleMargins(&self) -> bool {
        let mut result = <bool as ::std::default::Default>::default();
        let _rv = ((*self.vtable).GetHonorPageRuleMargins)(self, &mut result);
        debug_assert!(_rv.succeeded());
        result
    }


    /// ```text
    /// /** Whether @page rule margins should be honored or not. */
    /// ```
    ///

    /// `[infallible] attribute boolean honorPageRuleMargins;`
    #[inline]
    pub unsafe fn SetHonorPageRuleMargins(&self, aHonorPageRuleMargins: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetHonorPageRuleMargins)(self, aHonorPageRuleMargins)
    }


    /// ```text
    /// /** Whether to draw guidelines showing the margin settings */
    /// ```
    ///

    /// `[infallible] attribute boolean showMarginGuides;`
    #[inline]
    pub unsafe fn GetShowMarginGuides(&self) -> bool {
        let mut result = <bool as ::std::default::Default>::default();
        let _rv = ((*self.vtable).GetShowMarginGuides)(self, &mut result);
        debug_assert!(_rv.succeeded());
        result
    }


    /// ```text
    /// /** Whether to draw guidelines showing the margin settings */
    /// ```
    ///

    /// `[infallible] attribute boolean showMarginGuides;`
    #[inline]
    pub unsafe fn SetShowMarginGuides(&self, aShowMarginGuides: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetShowMarginGuides)(self, aShowMarginGuides)
    }


    /// ```text
    /// /** Whether whether the "print selection" radio button should be enabled
    ///    *  in the UI (i.e. whether there is an active selection) */
    /// ```
    ///

    /// `[infallible] attribute boolean isPrintSelectionRBEnabled;`
    #[inline]
    pub unsafe fn GetIsPrintSelectionRBEnabled(&self) -> bool {
        let mut result = <bool as ::std::default::Default>::default();
        let _rv = ((*self.vtable).GetIsPrintSelectionRBEnabled)(self, &mut result);
        debug_assert!(_rv.succeeded());
        result
    }


    /// ```text
    /// /** Whether whether the "print selection" radio button should be enabled
    ///    *  in the UI (i.e. whether there is an active selection) */
    /// ```
    ///

    /// `[infallible] attribute boolean isPrintSelectionRBEnabled;`
    #[inline]
    pub unsafe fn SetIsPrintSelectionRBEnabled(&self, aIsPrintSelectionRBEnabled: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetIsPrintSelectionRBEnabled)(self, aIsPrintSelectionRBEnabled)
    }


    /// ```text
    /// /** Whether to only print the selected nodes */
    /// ```
    ///

    /// `[infallible] attribute boolean printSelectionOnly;`
    #[inline]
    pub unsafe fn GetPrintSelectionOnly(&self) -> bool {
        let mut result = <bool as ::std::default::Default>::default();
        let _rv = ((*self.vtable).GetPrintSelectionOnly)(self, &mut result);
        debug_assert!(_rv.succeeded());
        result
    }


    /// ```text
    /// /** Whether to only print the selected nodes */
    /// ```
    ///

    /// `[infallible] attribute boolean printSelectionOnly;`
    #[inline]
    pub unsafe fn SetPrintSelectionOnly(&self, aPrintSelectionOnly: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetPrintSelectionOnly)(self, aPrintSelectionOnly)
    }



    /// `attribute AString title;`
    #[inline]
    pub unsafe fn GetTitle(&self, aTitle: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetTitle)(self, aTitle)
    }



    /// `attribute AString title;`
    #[inline]
    pub unsafe fn SetTitle(&self, aTitle: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).SetTitle)(self, aTitle)
    }



    /// `attribute AString docURL;`
    #[inline]
    pub unsafe fn GetDocURL(&self, aDocURL: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetDocURL)(self, aDocURL)
    }



    /// `attribute AString docURL;`
    #[inline]
    pub unsafe fn SetDocURL(&self, aDocURL: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).SetDocURL)(self, aDocURL)
    }



    /// `attribute AString headerStrLeft;`
    #[inline]
    pub unsafe fn GetHeaderStrLeft(&self, aHeaderStrLeft: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetHeaderStrLeft)(self, aHeaderStrLeft)
    }



    /// `attribute AString headerStrLeft;`
    #[inline]
    pub unsafe fn SetHeaderStrLeft(&self, aHeaderStrLeft: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).SetHeaderStrLeft)(self, aHeaderStrLeft)
    }



    /// `attribute AString headerStrCenter;`
    #[inline]
    pub unsafe fn GetHeaderStrCenter(&self, aHeaderStrCenter: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetHeaderStrCenter)(self, aHeaderStrCenter)
    }



    /// `attribute AString headerStrCenter;`
    #[inline]
    pub unsafe fn SetHeaderStrCenter(&self, aHeaderStrCenter: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).SetHeaderStrCenter)(self, aHeaderStrCenter)
    }



    /// `attribute AString headerStrRight;`
    #[inline]
    pub unsafe fn GetHeaderStrRight(&self, aHeaderStrRight: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetHeaderStrRight)(self, aHeaderStrRight)
    }



    /// `attribute AString headerStrRight;`
    #[inline]
    pub unsafe fn SetHeaderStrRight(&self, aHeaderStrRight: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).SetHeaderStrRight)(self, aHeaderStrRight)
    }



    /// `attribute AString footerStrLeft;`
    #[inline]
    pub unsafe fn GetFooterStrLeft(&self, aFooterStrLeft: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetFooterStrLeft)(self, aFooterStrLeft)
    }



    /// `attribute AString footerStrLeft;`
    #[inline]
    pub unsafe fn SetFooterStrLeft(&self, aFooterStrLeft: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).SetFooterStrLeft)(self, aFooterStrLeft)
    }



    /// `attribute AString footerStrCenter;`
    #[inline]
    pub unsafe fn GetFooterStrCenter(&self, aFooterStrCenter: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetFooterStrCenter)(self, aFooterStrCenter)
    }



    /// `attribute AString footerStrCenter;`
    #[inline]
    pub unsafe fn SetFooterStrCenter(&self, aFooterStrCenter: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).SetFooterStrCenter)(self, aFooterStrCenter)
    }



    /// `attribute AString footerStrRight;`
    #[inline]
    pub unsafe fn GetFooterStrRight(&self, aFooterStrRight: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetFooterStrRight)(self, aFooterStrRight)
    }



    /// `attribute AString footerStrRight;`
    #[inline]
    pub unsafe fn SetFooterStrRight(&self, aFooterStrRight: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).SetFooterStrRight)(self, aFooterStrRight)
    }



    /// `attribute boolean isCancelled;`
    #[inline]
    pub unsafe fn GetIsCancelled(&self, aIsCancelled: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetIsCancelled)(self, aIsCancelled)
    }



    /// `attribute boolean isCancelled;`
    #[inline]
    pub unsafe fn SetIsCancelled(&self, aIsCancelled: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetIsCancelled)(self, aIsCancelled)
    }



    /// `readonly attribute boolean saveOnCancel;`
    #[inline]
    pub unsafe fn GetSaveOnCancel(&self, aSaveOnCancel: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetSaveOnCancel)(self, aSaveOnCancel)
    }



    /// `attribute boolean printSilent;`
    #[inline]
    pub unsafe fn GetPrintSilent(&self, aPrintSilent: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetPrintSilent)(self, aPrintSilent)
    }



    /// `attribute boolean printSilent;`
    #[inline]
    pub unsafe fn SetPrintSilent(&self, aPrintSilent: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetPrintSilent)(self, aPrintSilent)
    }



    /// `attribute boolean shrinkToFit;`
    #[inline]
    pub unsafe fn GetShrinkToFit(&self, aShrinkToFit: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetShrinkToFit)(self, aShrinkToFit)
    }



    /// `attribute boolean shrinkToFit;`
    #[inline]
    pub unsafe fn SetShrinkToFit(&self, aShrinkToFit: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetShrinkToFit)(self, aShrinkToFit)
    }



    /// `attribute boolean showPrintProgress;`
    #[inline]
    pub unsafe fn GetShowPrintProgress(&self, aShowPrintProgress: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetShowPrintProgress)(self, aShowPrintProgress)
    }



    /// `attribute boolean showPrintProgress;`
    #[inline]
    pub unsafe fn SetShowPrintProgress(&self, aShowPrintProgress: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetShowPrintProgress)(self, aShowPrintProgress)
    }



    /// `attribute AString paperId;`
    #[inline]
    pub unsafe fn GetPaperId(&self, aPaperId: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetPaperId)(self, aPaperId)
    }



    /// `attribute AString paperId;`
    #[inline]
    pub unsafe fn SetPaperId(&self, aPaperId: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).SetPaperId)(self, aPaperId)
    }



    /// `attribute double paperWidth;`
    #[inline]
    pub unsafe fn GetPaperWidth(&self, aPaperWidth: *mut libc::c_double) -> ::nserror::nsresult {
        ((*self.vtable).GetPaperWidth)(self, aPaperWidth)
    }



    /// `attribute double paperWidth;`
    #[inline]
    pub unsafe fn SetPaperWidth(&self, aPaperWidth: libc::c_double) -> ::nserror::nsresult {
        ((*self.vtable).SetPaperWidth)(self, aPaperWidth)
    }



    /// `attribute double paperHeight;`
    #[inline]
    pub unsafe fn GetPaperHeight(&self, aPaperHeight: *mut libc::c_double) -> ::nserror::nsresult {
        ((*self.vtable).GetPaperHeight)(self, aPaperHeight)
    }



    /// `attribute double paperHeight;`
    #[inline]
    pub unsafe fn SetPaperHeight(&self, aPaperHeight: libc::c_double) -> ::nserror::nsresult {
        ((*self.vtable).SetPaperHeight)(self, aPaperHeight)
    }



    /// `attribute short paperSizeUnit;`
    #[inline]
    pub unsafe fn GetPaperSizeUnit(&self, aPaperSizeUnit: *mut i16) -> ::nserror::nsresult {
        ((*self.vtable).GetPaperSizeUnit)(self, aPaperSizeUnit)
    }



    /// `attribute short paperSizeUnit;`
    #[inline]
    pub unsafe fn SetPaperSizeUnit(&self, aPaperSizeUnit: i16) -> ::nserror::nsresult {
        ((*self.vtable).SetPaperSizeUnit)(self, aPaperSizeUnit)
    }



    /// `attribute boolean printReversed;`
    #[inline]
    pub unsafe fn GetPrintReversed(&self, aPrintReversed: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetPrintReversed)(self, aPrintReversed)
    }



    /// `attribute boolean printReversed;`
    #[inline]
    pub unsafe fn SetPrintReversed(&self, aPrintReversed: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetPrintReversed)(self, aPrintReversed)
    }



    /// `[infallible] attribute boolean printInColor;`
    #[inline]
    pub unsafe fn GetPrintInColor(&self) -> bool {
        let mut result = <bool as ::std::default::Default>::default();
        let _rv = ((*self.vtable).GetPrintInColor)(self, &mut result);
        debug_assert!(_rv.succeeded());
        result
    }



    /// `[infallible] attribute boolean printInColor;`
    #[inline]
    pub unsafe fn SetPrintInColor(&self, aPrintInColor: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetPrintInColor)(self, aPrintInColor)
    }



    /// `attribute long orientation;`
    #[inline]
    pub unsafe fn GetOrientation(&self, aOrientation: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetOrientation)(self, aOrientation)
    }



    /// `attribute long orientation;`
    #[inline]
    pub unsafe fn SetOrientation(&self, aOrientation: i32) -> ::nserror::nsresult {
        ((*self.vtable).SetOrientation)(self, aOrientation)
    }



    /// `attribute long numCopies;`
    #[inline]
    pub unsafe fn GetNumCopies(&self, aNumCopies: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetNumCopies)(self, aNumCopies)
    }



    /// `attribute long numCopies;`
    #[inline]
    pub unsafe fn SetNumCopies(&self, aNumCopies: i32) -> ::nserror::nsresult {
        ((*self.vtable).SetNumCopies)(self, aNumCopies)
    }


    /// ```text
    /// /**
    ///    * For numPagesPerSheet, we support these values: 1, 2, 4, 6, 9, 16.
    ///    *
    ///    * Unsupported values will be treated internally as 1 page per sheet, and
    ///    * will trigger assertion failures in debug builds.
    ///    */
    /// ```
    ///

    /// `attribute long numPagesPerSheet;`
    #[inline]
    pub unsafe fn GetNumPagesPerSheet(&self, aNumPagesPerSheet: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetNumPagesPerSheet)(self, aNumPagesPerSheet)
    }


    /// ```text
    /// /**
    ///    * For numPagesPerSheet, we support these values: 1, 2, 4, 6, 9, 16.
    ///    *
    ///    * Unsupported values will be treated internally as 1 page per sheet, and
    ///    * will trigger assertion failures in debug builds.
    ///    */
    /// ```
    ///

    /// `attribute long numPagesPerSheet;`
    #[inline]
    pub unsafe fn SetNumPagesPerSheet(&self, aNumPagesPerSheet: i32) -> ::nserror::nsresult {
        ((*self.vtable).SetNumPagesPerSheet)(self, aNumPagesPerSheet)
    }



    /// `attribute AString printerName;`
    #[inline]
    pub unsafe fn GetPrinterName(&self, aPrinterName: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetPrinterName)(self, aPrinterName)
    }



    /// `attribute AString printerName;`
    #[inline]
    pub unsafe fn SetPrinterName(&self, aPrinterName: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).SetPrinterName)(self, aPrinterName)
    }



    /// `attribute boolean printToFile;`
    #[inline]
    pub unsafe fn GetPrintToFile(&self, aPrintToFile: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetPrintToFile)(self, aPrintToFile)
    }



    /// `attribute boolean printToFile;`
    #[inline]
    pub unsafe fn SetPrintToFile(&self, aPrintToFile: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetPrintToFile)(self, aPrintToFile)
    }



    /// `attribute AString toFileName;`
    #[inline]
    pub unsafe fn GetToFileName(&self, aToFileName: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetToFileName)(self, aToFileName)
    }



    /// `attribute AString toFileName;`
    #[inline]
    pub unsafe fn SetToFileName(&self, aToFileName: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).SetToFileName)(self, aToFileName)
    }



    /// `attribute short outputFormat;`
    #[inline]
    pub unsafe fn GetOutputFormat(&self, aOutputFormat: *mut i16) -> ::nserror::nsresult {
        ((*self.vtable).GetOutputFormat)(self, aOutputFormat)
    }



    /// `attribute short outputFormat;`
    #[inline]
    pub unsafe fn SetOutputFormat(&self, aOutputFormat: i16) -> ::nserror::nsresult {
        ((*self.vtable).SetOutputFormat)(self, aOutputFormat)
    }



    /// `attribute long printPageDelay;`
    #[inline]
    pub unsafe fn GetPrintPageDelay(&self, aPrintPageDelay: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetPrintPageDelay)(self, aPrintPageDelay)
    }



    /// `attribute long printPageDelay;`
    #[inline]
    pub unsafe fn SetPrintPageDelay(&self, aPrintPageDelay: i32) -> ::nserror::nsresult {
        ((*self.vtable).SetPrintPageDelay)(self, aPrintPageDelay)
    }



    /// `attribute long resolution;`
    #[inline]
    pub unsafe fn GetResolution(&self, aResolution: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetResolution)(self, aResolution)
    }



    /// `attribute long resolution;`
    #[inline]
    pub unsafe fn SetResolution(&self, aResolution: i32) -> ::nserror::nsresult {
        ((*self.vtable).SetResolution)(self, aResolution)
    }



    /// `attribute long duplex;`
    #[inline]
    pub unsafe fn GetDuplex(&self, aDuplex: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetDuplex)(self, aDuplex)
    }



    /// `attribute long duplex;`
    #[inline]
    pub unsafe fn SetDuplex(&self, aDuplex: i32) -> ::nserror::nsresult {
        ((*self.vtable).SetDuplex)(self, aDuplex)
    }


    /// ```text
    /// /**
    ///    * This attribute tracks whether the PS has been initialized
    ///    * from a printer specified by the "printerName" attr.
    ///    * If a different name is set into the "printerName"
    ///    * attribute than the one it was initialized with the PS
    ///    * will then get intialized from that printer.
    ///    */
    /// ```
    ///

    /// `attribute boolean isInitializedFromPrinter;`
    #[inline]
    pub unsafe fn GetIsInitializedFromPrinter(&self, aIsInitializedFromPrinter: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetIsInitializedFromPrinter)(self, aIsInitializedFromPrinter)
    }


    /// ```text
    /// /**
    ///    * This attribute tracks whether the PS has been initialized
    ///    * from a printer specified by the "printerName" attr.
    ///    * If a different name is set into the "printerName"
    ///    * attribute than the one it was initialized with the PS
    ///    * will then get intialized from that printer.
    ///    */
    /// ```
    ///

    /// `attribute boolean isInitializedFromPrinter;`
    #[inline]
    pub unsafe fn SetIsInitializedFromPrinter(&self, aIsInitializedFromPrinter: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetIsInitializedFromPrinter)(self, aIsInitializedFromPrinter)
    }


    /// ```text
    /// /**
    ///    * This attribute tracks whether the PS has been initialized
    ///    * from prefs. If a different name is set into the "printerName"
    ///    * attribute than the one it was initialized with the PS
    ///    * will then get intialized from prefs again.
    ///    */
    /// ```
    ///

    /// `attribute boolean isInitializedFromPrefs;`
    #[inline]
    pub unsafe fn GetIsInitializedFromPrefs(&self, aIsInitializedFromPrefs: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetIsInitializedFromPrefs)(self, aIsInitializedFromPrefs)
    }


    /// ```text
    /// /**
    ///    * This attribute tracks whether the PS has been initialized
    ///    * from prefs. If a different name is set into the "printerName"
    ///    * attribute than the one it was initialized with the PS
    ///    * will then get intialized from prefs again.
    ///    */
    /// ```
    ///

    /// `attribute boolean isInitializedFromPrefs;`
    #[inline]
    pub unsafe fn SetIsInitializedFromPrefs(&self, aIsInitializedFromPrefs: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetIsInitializedFromPrefs)(self, aIsInitializedFromPrefs)
    }



    /// `[noscript] void SetMarginInTwips (in nsNativeIntMarginRef aMargin);`
    const _SetMarginInTwips: () = ();


    /// `[noscript] void SetEdgeInTwips (in nsNativeIntMarginRef aEdge);`
    const _SetEdgeInTwips: () = ();


    /// `[noscript,nostdcall,notxpcom] nsNativeIntMargin GetMarginInTwips ();`
    const _GetMarginInTwips: () = ();


    /// `[noscript,nostdcall,notxpcom] nsNativeIntMargin GetEdgeInTwips ();`
    const _GetEdgeInTwips: () = ();

    /// ```text
    /// /**
    ///    * We call this function so that anything that requires a run of the event loop
    ///    * can do so safely. The print dialog runs the event loop but in silent printing
    ///    * that doesn't happen.
    ///    *
    ///    * Either this or ShowPrintDialog (but not both) MUST be called by the print engine
    ///    * before printing, otherwise printing can fail on some platforms.
    ///    */
    /// ```
    ///

    /// `[noscript] void SetupSilentPrinting ();`
    #[inline]
    pub unsafe fn SetupSilentPrinting(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).SetupSilentPrinting)(self, )
    }


    /// ```text
    /// /**
    ///    * Sets/Gets the "unwriteable margin" for the page format.  This defines
    ///    * the boundary from which we'll measure the EdgeInTwips and MarginInTwips
    ///    * attributes, to place the headers and content, respectively.
    ///    *
    ///    * Note: Implementations of SetUnwriteableMarginInTwips should handle
    ///    * negative margin values by falling back on the system default for
    ///    * that margin.
    ///    */
    /// ```
    ///

    /// `[noscript] void SetUnwriteableMarginInTwips (in nsNativeIntMarginRef aEdge);`
    const _SetUnwriteableMarginInTwips: () = ();


    /// `[noscript,nostdcall,notxpcom] nsNativeIntMargin GetUnwriteableMarginInTwips ();`
    const _GetUnwriteableMarginInTwips: () = ();

    /// ```text
    /// /**
    ///    * Get more accurate print ranges from the superior interval
    ///    * (startPageRange, endPageRange). The aPages array is populated with a
    ///    * list of pairs (start, end), where the endpoints are included. The print
    ///    * ranges (start, end), must not overlap and must be in the
    ///    * (startPageRange, endPageRange) scope.
    ///    *
    ///    * If there are no print ranges the aPages array is empty.
    ///    */
    /// ```
    ///

    /// `attribute Array<long> pageRanges;`
    #[inline]
    pub unsafe fn GetPageRanges(&self, aPageRanges: *mut thin_vec::ThinVec<i32>) -> ::nserror::nsresult {
        ((*self.vtable).GetPageRanges)(self, aPageRanges)
    }


    /// ```text
    /// /**
    ///    * Get more accurate print ranges from the superior interval
    ///    * (startPageRange, endPageRange). The aPages array is populated with a
    ///    * list of pairs (start, end), where the endpoints are included. The print
    ///    * ranges (start, end), must not overlap and must be in the
    ///    * (startPageRange, endPageRange) scope.
    ///    *
    ///    * If there are no print ranges the aPages array is empty.
    ///    */
    /// ```
    ///

    /// `attribute Array<long> pageRanges;`
    #[inline]
    pub unsafe fn SetPageRanges(&self, aPageRanges: *const thin_vec::ThinVec<i32>) -> ::nserror::nsresult {
        ((*self.vtable).SetPageRanges)(self, aPageRanges)
    }


}


