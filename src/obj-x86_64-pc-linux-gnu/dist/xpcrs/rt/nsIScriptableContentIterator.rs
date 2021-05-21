//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/base/nsIScriptableContentIterator.idl
//


/// `interface nsIScriptableContentIterator : nsISupports`
///

/// ```text
/// /**
///  * nsIScriptableContentIterator is designed to testing concrete classes of
///  * ContentIteratorBase.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIScriptableContentIterator {
    vtable: *const nsIScriptableContentIteratorVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIScriptableContentIterator.
unsafe impl XpCom for nsIScriptableContentIterator {
    const IID: nsIID = nsID(0x9f25fb2a, 0x265f, 0x44f9,
        [0xa1, 0x22, 0x62, 0xbb, 0xf4, 0x43, 0x23, 0x9e]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIScriptableContentIterator {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIScriptableContentIterator.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIScriptableContentIteratorCoerce {
    /// Cheaply cast a value of this type from a `nsIScriptableContentIterator`.
    fn coerce_from(v: &nsIScriptableContentIterator) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIScriptableContentIteratorCoerce for nsIScriptableContentIterator {
    #[inline]
    fn coerce_from(v: &nsIScriptableContentIterator) -> &Self {
        v
    }
}

impl nsIScriptableContentIterator {
    /// Cast this `nsIScriptableContentIterator` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIScriptableContentIteratorCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIScriptableContentIterator {
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
impl<T: nsISupportsCoerce> nsIScriptableContentIteratorCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIScriptableContentIterator) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIScriptableContentIterator
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIScriptableContentIteratorVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void initWithRootNode (in nsIScriptableContentIterator_IteratorType aType, in Node aRoot); */
    pub InitWithRootNode: unsafe extern "system" fn (this: *const nsIScriptableContentIterator, aType:  u8, aRoot: *const libc::c_void) -> ::nserror::nsresult,

    /* void initWithRange (in nsIScriptableContentIterator_IteratorType aType, in Range aRange); */
    pub InitWithRange: unsafe extern "system" fn (this: *const nsIScriptableContentIterator, aType:  u8, aRange: *const libc::c_void) -> ::nserror::nsresult,

    /* void initWithPositions (in nsIScriptableContentIterator_IteratorType aType, in Node aStartContainer, in unsigned long aStartOffset, in Node aEndContainer, in unsigned long aEndOffset); */
    pub InitWithPositions: unsafe extern "system" fn (this: *const nsIScriptableContentIterator, aType:  u8, aStartContainer: *const libc::c_void, aStartOffset: u32, aEndContainer: *const libc::c_void, aEndOffset: u32) -> ::nserror::nsresult,

    /* void first (); */
    pub First: unsafe extern "system" fn (this: *const nsIScriptableContentIterator) -> ::nserror::nsresult,

    /* void last (); */
    pub Last: unsafe extern "system" fn (this: *const nsIScriptableContentIterator) -> ::nserror::nsresult,

    /* void next (); */
    pub Next: unsafe extern "system" fn (this: *const nsIScriptableContentIterator) -> ::nserror::nsresult,

    /* void prev (); */
    pub Prev: unsafe extern "system" fn (this: *const nsIScriptableContentIterator) -> ::nserror::nsresult,

    /* readonly attribute Node currentNode; */
    pub GetCurrentNode: unsafe extern "system" fn (this: *const nsIScriptableContentIterator, aCurrentNode: *mut *const libc::c_void) -> ::nserror::nsresult,

    /* readonly attribute bool isDone; */
    pub GetIsDone: unsafe extern "system" fn (this: *const nsIScriptableContentIterator, aIsDone: *mut bool) -> ::nserror::nsresult,

    /* void positionAt (in Node aNode); */
    pub PositionAt: unsafe extern "system" fn (this: *const nsIScriptableContentIterator, aNode: *const libc::c_void) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIScriptableContentIterator {

    /// ```text
    /// /**
    ///    * You need to call initWith*() first.  Then, the instance of this interface
    ///    * decides the type of iterator with its aType argument.  You can call
    ///    * initWith*() multiple times, but you need to keep setting same type as
    ///    * previous call.  If you set different type, these method with throw an
    ///    * exception.
    ///    */
    /// ```
    ///

    /// `void initWithRootNode (in nsIScriptableContentIterator_IteratorType aType, in Node aRoot);`
    #[inline]
    pub unsafe fn InitWithRootNode(&self, aType:  u8, aRoot: *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).InitWithRootNode)(self, aType, aRoot)
    }



    /// `void initWithRange (in nsIScriptableContentIterator_IteratorType aType, in Range aRange);`
    #[inline]
    pub unsafe fn InitWithRange(&self, aType:  u8, aRange: *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).InitWithRange)(self, aType, aRange)
    }



    /// `void initWithPositions (in nsIScriptableContentIterator_IteratorType aType, in Node aStartContainer, in unsigned long aStartOffset, in Node aEndContainer, in unsigned long aEndOffset);`
    #[inline]
    pub unsafe fn InitWithPositions(&self, aType:  u8, aStartContainer: *const libc::c_void, aStartOffset: u32, aEndContainer: *const libc::c_void, aEndOffset: u32) -> ::nserror::nsresult {
        ((*self.vtable).InitWithPositions)(self, aType, aStartContainer, aStartOffset, aEndContainer, aEndOffset)
    }



    /// `void first ();`
    #[inline]
    pub unsafe fn First(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).First)(self, )
    }



    /// `void last ();`
    #[inline]
    pub unsafe fn Last(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).Last)(self, )
    }



    /// `void next ();`
    #[inline]
    pub unsafe fn Next(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).Next)(self, )
    }



    /// `void prev ();`
    #[inline]
    pub unsafe fn Prev(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).Prev)(self, )
    }



    /// `readonly attribute Node currentNode;`
    #[inline]
    pub unsafe fn GetCurrentNode(&self, aCurrentNode: *mut *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).GetCurrentNode)(self, aCurrentNode)
    }



    /// `readonly attribute bool isDone;`
    #[inline]
    pub unsafe fn GetIsDone(&self, aIsDone: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetIsDone)(self, aIsDone)
    }



    /// `void positionAt (in Node aNode);`
    #[inline]
    pub unsafe fn PositionAt(&self, aNode: *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).PositionAt)(self, aNode)
    }


}


