//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/editor/nsIEditActionListener.idl
//


/// `interface nsIEditActionListener : nsISupports`
///

/// ```text
/// /**
///  * A generic editor action listener interface.
///  * <P>
///  * nsIEditActionListener is the interface used by applications wishing to be notified
///  * when the editor modifies the DOM tree.
///  *
///  * Note:  this is the wrong class to implement if you are interested in generic
///  * change notifications.  For generic notifications, you should implement
///  * nsIDocumentObserver.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIEditActionListener {
    vtable: *const nsIEditActionListenerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIEditActionListener.
unsafe impl XpCom for nsIEditActionListener {
    const IID: nsIID = nsID(0xb22907b1, 0xee93, 0x11d2,
        [0x8d, 0x50, 0x00, 0x00, 0x64, 0x65, 0x73, 0x74]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIEditActionListener {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIEditActionListener.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIEditActionListenerCoerce {
    /// Cheaply cast a value of this type from a `nsIEditActionListener`.
    fn coerce_from(v: &nsIEditActionListener) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIEditActionListenerCoerce for nsIEditActionListener {
    #[inline]
    fn coerce_from(v: &nsIEditActionListener) -> &Self {
        v
    }
}

impl nsIEditActionListener {
    /// Cast this `nsIEditActionListener` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIEditActionListenerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIEditActionListener {
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
impl<T: nsISupportsCoerce> nsIEditActionListenerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIEditActionListener) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIEditActionListener
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIEditActionListenerVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void DidDeleteNode (in Node aChild, in nsresult aResult); */
    pub DidDeleteNode: unsafe extern "system" fn (this: *const nsIEditActionListener, aChild: *const libc::c_void, aResult: ::nserror::nsresult) -> ::nserror::nsresult,

    /* void DidJoinNodes (in Node aLeftNode, in Node aRightNode, in Node aParent, in nsresult aResult); */
    pub DidJoinNodes: unsafe extern "system" fn (this: *const nsIEditActionListener, aLeftNode: *const libc::c_void, aRightNode: *const libc::c_void, aParent: *const libc::c_void, aResult: ::nserror::nsresult) -> ::nserror::nsresult,

    /* void DidInsertText (in CharacterData aTextNode, in long aOffset, in AString aString, in nsresult aResult); */
    pub DidInsertText: unsafe extern "system" fn (this: *const nsIEditActionListener, aTextNode: *const libc::c_void, aOffset: i32, aString: *const ::nsstring::nsAString, aResult: ::nserror::nsresult) -> ::nserror::nsresult,

    /* void WillDeleteText (in CharacterData aTextNode, in long aOffset, in long aLength); */
    pub WillDeleteText: unsafe extern "system" fn (this: *const nsIEditActionListener, aTextNode: *const libc::c_void, aOffset: i32, aLength: i32) -> ::nserror::nsresult,

    /* void WillDeleteRanges (in Array<Range> aRangesToDelete); */
    pub WillDeleteRanges: unsafe extern "system" fn (this: *const nsIEditActionListener, aRangesToDelete: *const thin_vec::ThinVec<*const libc::c_void>) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIEditActionListener {

    /// ```text
    /// /**
    ///    * Called after the editor deletes a node.
    ///    * @param aChild    The node to delete
    ///    * @param aResult   The result of the delete node operation.
    ///    */
    /// ```
    ///

    /// `void DidDeleteNode (in Node aChild, in nsresult aResult);`
    #[inline]
    pub unsafe fn DidDeleteNode(&self, aChild: *const libc::c_void, aResult: ::nserror::nsresult) -> ::nserror::nsresult {
        ((*self.vtable).DidDeleteNode)(self, aChild, aResult)
    }


    /// ```text
    /// /**
    ///    * Called after the editor joins 2 nodes.
    ///    * @param aLeftNode   This node will be merged into the right node
    ///    * @param aRightNode  The node that will be merged into.
    ///    *                    There is no requirement that the two nodes be of
    ///    *                    the same type.
    ///    * @param aParent     The parent of aRightNode
    ///    * @param aResult     The result of the join operation.
    ///    */
    /// ```
    ///

    /// `void DidJoinNodes (in Node aLeftNode, in Node aRightNode, in Node aParent, in nsresult aResult);`
    #[inline]
    pub unsafe fn DidJoinNodes(&self, aLeftNode: *const libc::c_void, aRightNode: *const libc::c_void, aParent: *const libc::c_void, aResult: ::nserror::nsresult) -> ::nserror::nsresult {
        ((*self.vtable).DidJoinNodes)(self, aLeftNode, aRightNode, aParent, aResult)
    }


    /// ```text
    /// /**
    ///    * Called after the editor inserts text.
    ///    * @param aTextNode   This node getting inserted text.
    ///    * @param aOffset     The offset in aTextNode to insert at.
    ///    * @param aString     The string that gets inserted.
    ///    * @param aResult     The result of the insert text operation.
    ///    */
    /// ```
    ///

    /// `void DidInsertText (in CharacterData aTextNode, in long aOffset, in AString aString, in nsresult aResult);`
    #[inline]
    pub unsafe fn DidInsertText(&self, aTextNode: *const libc::c_void, aOffset: i32, aString: *const ::nsstring::nsAString, aResult: ::nserror::nsresult) -> ::nserror::nsresult {
        ((*self.vtable).DidInsertText)(self, aTextNode, aOffset, aString, aResult)
    }


    /// ```text
    /// /**
    ///    * Called before the editor deletes text.
    ///    * @param aTextNode   This node getting text deleted.
    ///    * @param aOffset     The offset in aTextNode to delete at.
    ///    * @param aLength     The amount of text to delete.
    ///    */
    /// ```
    ///

    /// `void WillDeleteText (in CharacterData aTextNode, in long aOffset, in long aLength);`
    #[inline]
    pub unsafe fn WillDeleteText(&self, aTextNode: *const libc::c_void, aOffset: i32, aLength: i32) -> ::nserror::nsresult {
        ((*self.vtable).WillDeleteText)(self, aTextNode, aOffset, aLength)
    }


    /// ```text
    /// /**
    ///    * Called before the editor deletes the ranges.
    ///    * @param aRangesToDelete     The ranges to be deleted.
    ///    */
    /// ```
    ///

    /// `void WillDeleteRanges (in Array<Range> aRangesToDelete);`
    #[inline]
    pub unsafe fn WillDeleteRanges(&self, aRangesToDelete: *const thin_vec::ThinVec<*const libc::c_void>) -> ::nserror::nsresult {
        ((*self.vtable).WillDeleteRanges)(self, aRangesToDelete)
    }


}


