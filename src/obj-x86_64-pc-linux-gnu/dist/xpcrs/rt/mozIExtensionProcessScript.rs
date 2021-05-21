//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/extensions/mozIExtensionProcessScript.idl
//


/// `interface mozIExtensionProcessScript : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct mozIExtensionProcessScript {
    vtable: *const mozIExtensionProcessScriptVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for mozIExtensionProcessScript.
unsafe impl XpCom for mozIExtensionProcessScript {
    const IID: nsIID = nsID(0x6b09dc51, 0x6caa, 0x4ca7,
        [0x9d, 0x6d, 0x30, 0xc8, 0x72, 0x58, 0xa6, 0x30]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for mozIExtensionProcessScript {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from mozIExtensionProcessScript.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait mozIExtensionProcessScriptCoerce {
    /// Cheaply cast a value of this type from a `mozIExtensionProcessScript`.
    fn coerce_from(v: &mozIExtensionProcessScript) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl mozIExtensionProcessScriptCoerce for mozIExtensionProcessScript {
    #[inline]
    fn coerce_from(v: &mozIExtensionProcessScript) -> &Self {
        v
    }
}

impl mozIExtensionProcessScript {
    /// Cast this `mozIExtensionProcessScript` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: mozIExtensionProcessScriptCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for mozIExtensionProcessScript {
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
impl<T: nsISupportsCoerce> mozIExtensionProcessScriptCoerce for T {
    #[inline]
    fn coerce_from(v: &mozIExtensionProcessScript) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every mozIExtensionProcessScript
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct mozIExtensionProcessScriptVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void preloadContentScript (in nsISupports contentScript); */
    pub PreloadContentScript: unsafe extern "system" fn (this: *const mozIExtensionProcessScript, contentScript: *const nsISupports) -> ::nserror::nsresult,

    /* Promise loadContentScript (in WebExtensionContentScript contentScript, in mozIDOMWindow window); */
    /// Unable to generate binding because `specialtype promise unsupported`
    pub LoadContentScript: *const ::libc::c_void,

    /* void initExtensionDocument (in nsISupports extension, in Document doc, in bool privileged); */
    pub InitExtensionDocument: unsafe extern "system" fn (this: *const mozIExtensionProcessScript, extension: *const nsISupports, doc: *const libc::c_void, privileged: bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl mozIExtensionProcessScript {


    /// `void preloadContentScript (in nsISupports contentScript);`
    #[inline]
    pub unsafe fn PreloadContentScript(&self, contentScript: *const nsISupports) -> ::nserror::nsresult {
        ((*self.vtable).PreloadContentScript)(self, contentScript)
    }



    /// `Promise loadContentScript (in WebExtensionContentScript contentScript, in mozIDOMWindow window);`
    const _LoadContentScript: () = ();


    /// `void initExtensionDocument (in nsISupports extension, in Document doc, in bool privileged);`
    #[inline]
    pub unsafe fn InitExtensionDocument(&self, extension: *const nsISupports, doc: *const libc::c_void, privileged: bool) -> ::nserror::nsresult {
        ((*self.vtable).InitExtensionDocument)(self, extension, doc, privileged)
    }


}


