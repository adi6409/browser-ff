//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/uriloader/base/nsIDocumentLoader.idl
//


/// `interface nsIDocumentLoader : nsISupports`
///

/// ```text
/// /**
///  * An nsIDocumentLoader is an interface responsible for tracking groups of
///  * loads that belong together (images, external scripts, etc) and subdocuments
///  * (<iframe>, <frame>, etc). It is also responsible for sending
///  * nsIWebProgressListener notifications.
///  * XXXbz this interface should go away, we think...
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIDocumentLoader {
    vtable: *const nsIDocumentLoaderVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIDocumentLoader.
unsafe impl XpCom for nsIDocumentLoader {
    const IID: nsIID = nsID(0xbbe961ee, 0x59e9, 0x42bb,
        [0xbe, 0x50, 0x03, 0x31, 0x97, 0x9b, 0xb7, 0x9f]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIDocumentLoader {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIDocumentLoader.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIDocumentLoaderCoerce {
    /// Cheaply cast a value of this type from a `nsIDocumentLoader`.
    fn coerce_from(v: &nsIDocumentLoader) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIDocumentLoaderCoerce for nsIDocumentLoader {
    #[inline]
    fn coerce_from(v: &nsIDocumentLoader) -> &Self {
        v
    }
}

impl nsIDocumentLoader {
    /// Cast this `nsIDocumentLoader` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIDocumentLoaderCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIDocumentLoader {
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
impl<T: nsISupportsCoerce> nsIDocumentLoaderCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDocumentLoader) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIDocumentLoader
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIDocumentLoaderVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void stop (); */
    pub Stop: unsafe extern "system" fn (this: *const nsIDocumentLoader) -> ::nserror::nsresult,

    /* readonly attribute nsISupports container; */
    pub GetContainer: unsafe extern "system" fn (this: *const nsIDocumentLoader, aContainer: *mut *const nsISupports) -> ::nserror::nsresult,

    /* readonly attribute nsILoadGroup loadGroup; */
    pub GetLoadGroup: unsafe extern "system" fn (this: *const nsIDocumentLoader, aLoadGroup: *mut*const nsILoadGroup) -> ::nserror::nsresult,

    /* readonly attribute nsIChannel documentChannel; */
    pub GetDocumentChannel: unsafe extern "system" fn (this: *const nsIDocumentLoader, aDocumentChannel: *mut*const nsIChannel) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIDocumentLoader {


    /// `void stop ();`
    #[inline]
    pub unsafe fn Stop(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).Stop)(self, )
    }



    /// `readonly attribute nsISupports container;`
    #[inline]
    pub unsafe fn GetContainer(&self, aContainer: *mut *const nsISupports) -> ::nserror::nsresult {
        ((*self.vtable).GetContainer)(self, aContainer)
    }



    /// `readonly attribute nsILoadGroup loadGroup;`
    #[inline]
    pub unsafe fn GetLoadGroup(&self, aLoadGroup: *mut*const nsILoadGroup) -> ::nserror::nsresult {
        ((*self.vtable).GetLoadGroup)(self, aLoadGroup)
    }



    /// `readonly attribute nsIChannel documentChannel;`
    #[inline]
    pub unsafe fn GetDocumentChannel(&self, aDocumentChannel: *mut*const nsIChannel) -> ::nserror::nsresult {
        ((*self.vtable).GetDocumentChannel)(self, aDocumentChannel)
    }


}


