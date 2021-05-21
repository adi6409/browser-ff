//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/widget/nsIPrinter.idl
//


/// `interface nsIPrinterInfo : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIPrinterInfo {
    vtable: *const nsIPrinterInfoVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIPrinterInfo.
unsafe impl XpCom for nsIPrinterInfo {
    const IID: nsIID = nsID(0x855ae9dd, 0x62a4, 0x64aa,
        [0x9c, 0x60, 0xb1, 0x07, 0x8f, 0xf0, 0x28, 0xf1]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIPrinterInfo {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIPrinterInfo.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIPrinterInfoCoerce {
    /// Cheaply cast a value of this type from a `nsIPrinterInfo`.
    fn coerce_from(v: &nsIPrinterInfo) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIPrinterInfoCoerce for nsIPrinterInfo {
    #[inline]
    fn coerce_from(v: &nsIPrinterInfo) -> &Self {
        v
    }
}

impl nsIPrinterInfo {
    /// Cast this `nsIPrinterInfo` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIPrinterInfoCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIPrinterInfo {
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
impl<T: nsISupportsCoerce> nsIPrinterInfoCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPrinterInfo) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIPrinterInfo
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIPrinterInfoVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute Array<nsIPaper> paperList; */
    pub GetPaperList: unsafe extern "system" fn (this: *const nsIPrinterInfo, aPaperList: *mut thin_vec::ThinVec<RefPtr<nsIPaper>>) -> ::nserror::nsresult,

    /* readonly attribute nsIPrintSettings defaultSettings; */
    pub GetDefaultSettings: unsafe extern "system" fn (this: *const nsIPrinterInfo, aDefaultSettings: *mut *const nsIPrintSettings) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIPrinterInfo {

    /// ```text
    /// /**
    ///    * An array of nsIPaper instances that represents the available paper sizes.
    ///    */
    /// ```
    ///

    /// `readonly attribute Array<nsIPaper> paperList;`
    #[inline]
    pub unsafe fn GetPaperList(&self, aPaperList: *mut thin_vec::ThinVec<RefPtr<nsIPaper>>) -> ::nserror::nsresult {
        ((*self.vtable).GetPaperList)(self, aPaperList)
    }


    /// ```text
    /// /**
    ///    * nsIPrintSettings object containing the default settings for a printer.
    ///    */
    /// ```
    ///

    /// `readonly attribute nsIPrintSettings defaultSettings;`
    #[inline]
    pub unsafe fn GetDefaultSettings(&self, aDefaultSettings: *mut *const nsIPrintSettings) -> ::nserror::nsresult {
        ((*self.vtable).GetDefaultSettings)(self, aDefaultSettings)
    }


}


/// `interface nsIPrinter : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIPrinter {
    vtable: *const nsIPrinterVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIPrinter.
unsafe impl XpCom for nsIPrinter {
    const IID: nsIID = nsID(0xd2dde9bb, 0xdf86, 0x469c,
        [0xbf, 0xcc, 0xfd, 0x95, 0xa4, 0x4b, 0x1d, 0xb8]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIPrinter {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIPrinter.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIPrinterCoerce {
    /// Cheaply cast a value of this type from a `nsIPrinter`.
    fn coerce_from(v: &nsIPrinter) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIPrinterCoerce for nsIPrinter {
    #[inline]
    fn coerce_from(v: &nsIPrinter) -> &Self {
        v
    }
}

impl nsIPrinter {
    /// Cast this `nsIPrinter` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIPrinterCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIPrinter {
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
impl<T: nsISupportsCoerce> nsIPrinterCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPrinter) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIPrinter
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIPrinterVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute AString name; */
    pub GetName: unsafe extern "system" fn (this: *const nsIPrinter, aName: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute AString systemName; */
    pub GetSystemName: unsafe extern "system" fn (this: *const nsIPrinter, aSystemName: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* [implicit_jscontext] readonly attribute Promise printerInfo; */
    /// Unable to generate binding because `specialtype promise unsupported`
    pub GetPrinterInfo: *const ::libc::c_void,

    /* [implicit_jscontext] readonly attribute Promise supportsDuplex; */
    /// Unable to generate binding because `specialtype promise unsupported`
    pub GetSupportsDuplex: *const ::libc::c_void,

    /* [implicit_jscontext] readonly attribute Promise supportsColor; */
    /// Unable to generate binding because `specialtype promise unsupported`
    pub GetSupportsColor: *const ::libc::c_void,

    /* [implicit_jscontext] readonly attribute Promise supportsMonochrome; */
    /// Unable to generate binding because `specialtype promise unsupported`
    pub GetSupportsMonochrome: *const ::libc::c_void,

    /* [implicit_jscontext] readonly attribute Promise supportsCollation; */
    /// Unable to generate binding because `specialtype promise unsupported`
    pub GetSupportsCollation: *const ::libc::c_void,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIPrinter {

    /// ```text
    /// /**
    ///    * The name of the printer.
    ///    */
    /// ```
    ///

    /// `readonly attribute AString name;`
    #[inline]
    pub unsafe fn GetName(&self, aName: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetName)(self, aName)
    }


    /// ```text
    /// /**
    ///    * The system name of the printer.
    ///    *
    ///    * This may be faster for lookup in nsIPrinterList functions, but will only
    ///    * work for functions that will accept the system name.
    ///    */
    /// ```
    ///

    /// `readonly attribute AString systemName;`
    #[inline]
    pub unsafe fn GetSystemName(&self, aSystemName: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetSystemName)(self, aSystemName)
    }


    /// ```text
    /// /**
    ///    * Returns a Promise that resolves to a nsIPrinterInfo.
    ///    * This will contain the default printer settings, and the list of paper
    ///    * sizes supported by the printer.
    ///    */
    /// ```
    ///

    /// `[implicit_jscontext] readonly attribute Promise printerInfo;`
    const _GetPrinterInfo: () = ();

    /// ```text
    /// /**
    ///    * Returns a Promise that resolves to true or false to indicate whether this
    ///    * printer supports duplex printing.
    ///    */
    /// ```
    ///

    /// `[implicit_jscontext] readonly attribute Promise supportsDuplex;`
    const _GetSupportsDuplex: () = ();

    /// ```text
    /// /**
    ///    * Returns a Promise that resolves to true or false to indicate whether this
    ///    * printer supports color printing.
    ///    */
    /// ```
    ///

    /// `[implicit_jscontext] readonly attribute Promise supportsColor;`
    const _GetSupportsColor: () = ();

    /// ```text
    /// /**
    ///    * Returns a Promise that resolves to true or false to indicate whether this
    ///    * printer supports monochrome printing.
    ///    */
    /// ```
    ///

    /// `[implicit_jscontext] readonly attribute Promise supportsMonochrome;`
    const _GetSupportsMonochrome: () = ();

    /// ```text
    /// /**
    ///    * Returns a Promise that resolves to true or false to indicate whether this
    ///    * printer supports collation.
    ///    */
    /// ```
    ///

    /// `[implicit_jscontext] readonly attribute Promise supportsCollation;`
    const _GetSupportsCollation: () = ();

}


