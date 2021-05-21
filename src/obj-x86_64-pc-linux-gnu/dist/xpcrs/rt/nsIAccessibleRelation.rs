//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/accessible/interfaces/nsIAccessibleRelation.idl
//


/// `interface nsIAccessibleRelation : nsISupports`
///

/// ```text
/// /**
///  * This interface gives access to an accessible's set of relations.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIAccessibleRelation {
    vtable: *const nsIAccessibleRelationVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIAccessibleRelation.
unsafe impl XpCom for nsIAccessibleRelation {
    const IID: nsIID = nsID(0x55b308c4, 0x2ae4, 0x46bc,
        [0xb4, 0xcd, 0x4d, 0x43, 0x70, 0xe0, 0xa6, 0x60]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIAccessibleRelation {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIAccessibleRelation.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIAccessibleRelationCoerce {
    /// Cheaply cast a value of this type from a `nsIAccessibleRelation`.
    fn coerce_from(v: &nsIAccessibleRelation) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIAccessibleRelationCoerce for nsIAccessibleRelation {
    #[inline]
    fn coerce_from(v: &nsIAccessibleRelation) -> &Self {
        v
    }
}

impl nsIAccessibleRelation {
    /// Cast this `nsIAccessibleRelation` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIAccessibleRelationCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIAccessibleRelation {
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
impl<T: nsISupportsCoerce> nsIAccessibleRelationCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAccessibleRelation) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIAccessibleRelation
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIAccessibleRelationVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute unsigned long relationType; */
    pub GetRelationType: unsafe extern "system" fn (this: *const nsIAccessibleRelation, aRelationType: *mut u32) -> ::nserror::nsresult,

    /* readonly attribute unsigned long targetsCount; */
    pub GetTargetsCount: unsafe extern "system" fn (this: *const nsIAccessibleRelation, aTargetsCount: *mut u32) -> ::nserror::nsresult,

    /* nsIAccessible getTarget (in unsigned long index); */
    pub GetTarget: unsafe extern "system" fn (this: *const nsIAccessibleRelation, index: u32, _retval: *mut*const nsIAccessible) -> ::nserror::nsresult,

    /* nsIArray getTargets (); */
    pub GetTargets: unsafe extern "system" fn (this: *const nsIAccessibleRelation, _retval: *mut *const nsIArray) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIAccessibleRelation {
    /// ```text
    /// /**
    ///    * This object is labelled by a target object.
    ///    */
    /// ```
    ///

    pub const RELATION_LABELLED_BY: i64 = 0;

    /// ```text
    /// /**
    ///    * This object is label for a target object.
    ///    */
    /// ```
    ///

    pub const RELATION_LABEL_FOR: i64 = 1;

    /// ```text
    /// /**
    ///    * This object is described by the target object.
    ///    */
    /// ```
    ///

    pub const RELATION_DESCRIBED_BY: i64 = 2;

    /// ```text
    /// /**
    ///    * This object is describes the target object.
    ///    */
    /// ```
    ///

    pub const RELATION_DESCRIPTION_FOR: i64 = 3;

    /// ```text
    /// /**
    ///    * This object is a child of a target object.
    ///    */
    /// ```
    ///

    pub const RELATION_NODE_CHILD_OF: i64 = 4;

    /// ```text
    /// /**
    ///    * This object is a parent of a target object. A dual relation to
    ///    * RELATION_NODE_CHILD_OF
    ///    */
    /// ```
    ///

    pub const RELATION_NODE_PARENT_OF: i64 = 5;

    /// ```text
    /// /**
    ///    * Some attribute of this object is affected by a target object.
    ///    */
    /// ```
    ///

    pub const RELATION_CONTROLLED_BY: i64 = 6;

    /// ```text
    /// /**
    ///    * This object is interactive and controls some attribute of a target object.
    ///    */
    /// ```
    ///

    pub const RELATION_CONTROLLER_FOR: i64 = 7;

    /// ```text
    /// /**
    ///    * Content flows from this object to a target object, i.e. has content that
    ///    * flows logically to another object in a sequential way, e.g. text flow.
    ///    */
    /// ```
    ///

    pub const RELATION_FLOWS_TO: i64 = 8;

    /// ```text
    /// /**
    ///    * Content flows to this object from a target object, i.e. has content that
    ///    * flows logically from another object in a sequential way, e.g. text flow.
    ///    */
    /// ```
    ///

    pub const RELATION_FLOWS_FROM: i64 = 9;

    /// ```text
    /// /**
    ///    * This object is a member of a group of one or more objects. When there is
    ///    * more than one object in the group each member may have one and the same
    ///    * target, e.g. a grouping object.  It is also possible that each member has
    ///    * multiple additional targets, e.g. one for every other member in the group.
    ///    */
    /// ```
    ///

    pub const RELATION_MEMBER_OF: i64 = 10;

    /// ```text
    /// /**
    ///    * This object is a sub window of a target object.
    ///    */
    /// ```
    ///

    pub const RELATION_SUBWINDOW_OF: i64 = 11;

    /// ```text
    /// /**
    ///    * This object embeds a target object. This relation can be used on the
    ///    * OBJID_CLIENT accessible for a top level window to show where the content
    ///    * areas are.
    ///    */
    /// ```
    ///

    pub const RELATION_EMBEDS: i64 = 12;

    /// ```text
    /// /**
    ///    * This object is embedded by a target object.
    ///    */
    /// ```
    ///

    pub const RELATION_EMBEDDED_BY: i64 = 13;

    /// ```text
    /// /**
    ///    * This object is a transient component related to the target object. When
    ///    * this object is activated the target object doesn't lose focus.
    ///    */
    /// ```
    ///

    pub const RELATION_POPUP_FOR: i64 = 14;

    /// ```text
    /// /**
    ///    * This object is a parent window of the target object.
    ///    */
    /// ```
    ///

    pub const RELATION_PARENT_WINDOW_OF: i64 = 15;

    /// ```text
    /// /**
    ///    * Part of a form/dialog with a related default button. It is used for
    ///    * MSAA/XPCOM, it isn't for IA2 or ATK.
    ///    */
    /// ```
    ///

    pub const RELATION_DEFAULT_BUTTON: i64 = 16;

    /// ```text
    /// /**
    ///    * The target object is the containing document object.
    ///    */
    /// ```
    ///

    pub const RELATION_CONTAINING_DOCUMENT: i64 = 17;

    /// ```text
    /// /**
    ///    * The target object is the topmost containing document object in the tab pane.
    ///    */
    /// ```
    ///

    pub const RELATION_CONTAINING_TAB_PANE: i64 = 18;

    /// ```text
    /// /**
    ///    * The target object is the containing window object.
    ///    */
    /// ```
    ///

    pub const RELATION_CONTAINING_WINDOW: i64 = 19;

    /// ```text
    /// /**
    ///    * The target object is the containing application object.
    ///    */
    /// ```
    ///

    pub const RELATION_CONTAINING_APPLICATION: i64 = 20;

    /// ```text
    /// /**
    ///    * The target object provides the detailed, extended description for this
    ///    * object. It provides more detailed information than would normally be
    ///    * provided using the DESCRIBED_BY relation. A common use for this relation is
    ///    * in digital publishing where an extended description needs to be conveyed in
    ///    * a book that requires structural markup or the embedding of other technology
    ///    * to provide illustrative content.
    ///    */
    /// ```
    ///

    pub const RELATION_DETAILS: i64 = 21;

    /// ```text
    /// /**
    ///    * This object provides the detailed, extended description for the target
    ///    * object. See DETAILS relation.
    ///    */
    /// ```
    ///

    pub const RELATION_DETAILS_FOR: i64 = 22;

    /// ```text
    /// /**
    ///    * The target object is the error message for this object.
    ///    */
    /// ```
    ///

    pub const RELATION_ERRORMSG: i64 = 23;

    /// ```text
    /// /**
    ///    * This object is the error message for the target object.
    ///    */
    /// ```
    ///

    pub const RELATION_ERRORMSG_FOR: i64 = 24;

    /// ```text
    /// /**
    ///    * Returns the type of the relation.
    ///    */
    /// ```
    ///

    /// `readonly attribute unsigned long relationType;`
    #[inline]
    pub unsafe fn GetRelationType(&self, aRelationType: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetRelationType)(self, aRelationType)
    }


    /// ```text
    /// /**
    ///    * Returns the number of targets for this relation.
    ///    */
    /// ```
    ///

    /// `readonly attribute unsigned long targetsCount;`
    #[inline]
    pub unsafe fn GetTargetsCount(&self, aTargetsCount: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetTargetsCount)(self, aTargetsCount)
    }


    /// ```text
    /// /**
    ///    * Returns one accessible relation target.
    ///    * @param index - 0 based index of relation target.
    ///    */
    /// ```
    ///

    /// `nsIAccessible getTarget (in unsigned long index);`
    #[inline]
    pub unsafe fn GetTarget(&self, index: u32, _retval: *mut*const nsIAccessible) -> ::nserror::nsresult {
        ((*self.vtable).GetTarget)(self, index, _retval)
    }


    /// ```text
    /// /**
    ///    * Returns multiple accessible relation targets.
    ///    */
    /// ```
    ///

    /// `nsIArray getTargets ();`
    #[inline]
    pub unsafe fn GetTargets(&self, _retval: *mut *const nsIArray) -> ::nserror::nsresult {
        ((*self.vtable).GetTargets)(self, _retval)
    }


}


