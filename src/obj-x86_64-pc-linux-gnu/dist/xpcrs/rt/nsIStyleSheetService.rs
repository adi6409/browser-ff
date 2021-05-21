//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/layout/base/nsIStyleSheetService.idl
//


/// `interface nsIStyleSheetService : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIStyleSheetService {
    vtable: *const nsIStyleSheetServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIStyleSheetService.
unsafe impl XpCom for nsIStyleSheetService {
    const IID: nsIID = nsID(0x4de68896, 0xe8eb, 0x41de,
        [0x82, 0x37, 0xa7, 0x97, 0xb5, 0x70, 0xac, 0x4a]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIStyleSheetService {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIStyleSheetService.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIStyleSheetServiceCoerce {
    /// Cheaply cast a value of this type from a `nsIStyleSheetService`.
    fn coerce_from(v: &nsIStyleSheetService) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIStyleSheetServiceCoerce for nsIStyleSheetService {
    #[inline]
    fn coerce_from(v: &nsIStyleSheetService) -> &Self {
        v
    }
}

impl nsIStyleSheetService {
    /// Cast this `nsIStyleSheetService` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIStyleSheetServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIStyleSheetService {
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
impl<T: nsISupportsCoerce> nsIStyleSheetServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIStyleSheetService) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIStyleSheetService
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIStyleSheetServiceVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void loadAndRegisterSheet (in nsIURI sheetURI, in unsigned long type); */
    pub LoadAndRegisterSheet: unsafe extern "system" fn (this: *const nsIStyleSheetService, sheetURI: *const nsIURI, type_: u32) -> ::nserror::nsresult,

    /* boolean sheetRegistered (in nsIURI sheetURI, in unsigned long type); */
    pub SheetRegistered: unsafe extern "system" fn (this: *const nsIStyleSheetService, sheetURI: *const nsIURI, type_: u32, _retval: *mut bool) -> ::nserror::nsresult,

    /* nsIPreloadedStyleSheet preloadSheet (in nsIURI sheetURI, in unsigned long type); */
    pub PreloadSheet: unsafe extern "system" fn (this: *const nsIStyleSheetService, sheetURI: *const nsIURI, type_: u32, _retval: *mut*const nsIPreloadedStyleSheet) -> ::nserror::nsresult,

    /* [implicit_jscontext] jsval preloadSheetAsync (in nsIURI sheetURI, in unsigned long type); */
    /// Unable to generate binding because `jscontext is unsupported`
    pub PreloadSheetAsync: *const ::libc::c_void,

    /* void unregisterSheet (in nsIURI sheetURI, in unsigned long type); */
    pub UnregisterSheet: unsafe extern "system" fn (this: *const nsIStyleSheetService, sheetURI: *const nsIURI, type_: u32) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIStyleSheetService {

    pub const AGENT_SHEET: i64 = 0;


    pub const USER_SHEET: i64 = 1;


    pub const AUTHOR_SHEET: i64 = 2;

    /// ```text
    /// /**
    ///    * Synchronously loads a style sheet from |sheetURI| and adds it to the list
    ///    * of user or agent style sheets.
    ///    *
    ///    * A user sheet loaded via this API will come before userContent.css and
    ///    * userChrome.css in the cascade (so the rules in it will have lower
        ///    * precedence than rules in those sheets).
    ///    *
    ///    * An agent sheet loaded via this API will come after ua.css in the cascade
    ///    * (so the rules in it will have higher precedence than rules in ua.css).
    ///    *
    ///    * The relative ordering of two user or two agent sheets loaded via
    ///    * this API is undefined.
    ///    *
    ///    * Sheets added via this API take effect on all documents, including
    ///    * already-loaded ones, immediately.
    ///    */
    /// ```
    ///

    /// `void loadAndRegisterSheet (in nsIURI sheetURI, in unsigned long type);`
    #[inline]
    pub unsafe fn LoadAndRegisterSheet(&self, sheetURI: *const nsIURI, type_: u32) -> ::nserror::nsresult {
        ((*self.vtable).LoadAndRegisterSheet)(self, sheetURI, type_)
    }


    /// ```text
    /// /**
    ///    * Returns true if a style sheet at |sheetURI| has previously been
    ///    * added to the list of style sheets specified by |type|.
    ///    */
    /// ```
    ///

    /// `boolean sheetRegistered (in nsIURI sheetURI, in unsigned long type);`
    #[inline]
    pub unsafe fn SheetRegistered(&self, sheetURI: *const nsIURI, type_: u32, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).SheetRegistered)(self, sheetURI, type_, _retval)
    }


    /// ```text
    /// /**
    ///    * Synchronously loads a style sheet from |sheetURI| and returns the
    ///    * new style sheet object. Can be used with nsIDOMWindowUtils.addSheet.
    ///    */
    /// ```
    ///

    /// `nsIPreloadedStyleSheet preloadSheet (in nsIURI sheetURI, in unsigned long type);`
    #[inline]
    pub unsafe fn PreloadSheet(&self, sheetURI: *const nsIURI, type_: u32, _retval: *mut*const nsIPreloadedStyleSheet) -> ::nserror::nsresult {
        ((*self.vtable).PreloadSheet)(self, sheetURI, type_, _retval)
    }


    /// ```text
    /// /**
    ///    * Asynchronously loads a style sheet from |sheetURI| and returns a Promise
    ///    * which resolves to the new style sheet object, which can be used with
    ///    * nsIDOMWindowUtils.addSheet, when it has completed loading.
    ///    */
    /// ```
    ///

    /// `[implicit_jscontext] jsval preloadSheetAsync (in nsIURI sheetURI, in unsigned long type);`
    const _PreloadSheetAsync: () = ();

    /// ```text
    /// /**
    ///    * Remove the style sheet at |sheetURI| from the list of style sheets
    ///    * specified by |type|.  The removal takes effect immediately, even for
    ///    * already-loaded documents.
    ///    */
    /// ```
    ///

    /// `void unregisterSheet (in nsIURI sheetURI, in unsigned long type);`
    #[inline]
    pub unsafe fn UnregisterSheet(&self, sheetURI: *const nsIURI, type_: u32) -> ::nserror::nsresult {
        ((*self.vtable).UnregisterSheet)(self, sheetURI, type_)
    }


}


