//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/accessible/interfaces/nsIAccessible.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIAccessible",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute nsIAccessible parent; */
                    Method {
                        name: "GetParent",
                        params: &[Param { name: "aParent", ty: "*mut *const nsIAccessible" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsIAccessible nextSibling; */
                    Method {
                        name: "GetNextSibling",
                        params: &[Param { name: "aNextSibling", ty: "*mut *const nsIAccessible" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsIAccessible previousSibling; */
                    Method {
                        name: "GetPreviousSibling",
                        params: &[Param { name: "aPreviousSibling", ty: "*mut *const nsIAccessible" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsIAccessible firstChild; */
                    Method {
                        name: "GetFirstChild",
                        params: &[Param { name: "aFirstChild", ty: "*mut *const nsIAccessible" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsIAccessible lastChild; */
                    Method {
                        name: "GetLastChild",
                        params: &[Param { name: "aLastChild", ty: "*mut *const nsIAccessible" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsIArray children; */
                    Method {
                        name: "GetChildren",
                        params: &[Param { name: "aChildren", ty: "*mut *const nsIArray" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute long childCount; */
                    Method {
                        name: "GetChildCount",
                        params: &[Param { name: "aChildCount", ty: "*mut i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute long indexInParent; */
                    Method {
                        name: "GetIndexInParent",
                        params: &[Param { name: "aIndexInParent", ty: "*mut i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute long long uniqueID; */
                    Method {
                        name: "GetUniqueID",
                        params: &[Param { name: "aUniqueID", ty: "*mut i64" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute Node DOMNode; */
                    Method {
                        name: "GetDOMNode",
                        params: &[Param { name: "aDOMNode", ty: "*mut *const libc::c_void" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute AString id; */
                    Method {
                        name: "GetId",
                        params: &[Param { name: "aId", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsIAccessibleDocument document; */
                    Method {
                        name: "GetDocument",
                        params: &[Param { name: "aDocument", ty: "*mut*const nsIAccessibleDocument" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsIAccessibleDocument rootDocument; */
                    Method {
                        name: "GetRootDocument",
                        params: &[Param { name: "aRootDocument", ty: "*mut*const nsIAccessibleDocument" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute AString language; */
                    Method {
                        name: "GetLanguage",
                        params: &[Param { name: "aLanguage", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute AString name; */
                    Method {
                        name: "GetName",
                        params: &[Param { name: "aName", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute AString value; */
                    Method {
                        name: "GetValue",
                        params: &[Param { name: "aValue", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute AString description; */
                    Method {
                        name: "GetDescription",
                        params: &[Param { name: "aDescription", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute AString accessKey; */
                    Method {
                        name: "GetAccessKey",
                        params: &[Param { name: "aAccessKey", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute AString keyboardShortcut; */
                    Method {
                        name: "GetKeyboardShortcut",
                        params: &[Param { name: "aKeyboardShortcut", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute unsigned long role; */
                    Method {
                        name: "GetRole",
                        params: &[Param { name: "aRole", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void getState (out unsigned long aState, out unsigned long aExtraState); */
                    Method {
                        name: "GetState",
                        params: &[Param { name: "aState", ty: "*mut u32" }, Param { name: "aExtraState", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute AString help; */
                    Method {
                        name: "GetHelp",
                        params: &[Param { name: "aHelp", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsIAccessible focusedChild; */
                    Method {
                        name: "GetFocusedChild",
                        params: &[Param { name: "aFocusedChild", ty: "*mut *const nsIAccessible" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsIPersistentProperties attributes; */
                    Method {
                        name: "GetAttributes",
                        params: &[Param { name: "aAttributes", ty: "*mut*const nsIPersistentProperties" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsISupports nativeInterface; */
                    Method {
                        name: "GetNativeInterface",
                        params: &[Param { name: "aNativeInterface", ty: "*mut *const nsISupports" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void groupPosition (out long aGroupLevel, out long aSimilarItemsInGroup, out long aPositionInGroup); */
                    Method {
                        name: "GroupPosition",
                        params: &[Param { name: "aGroupLevel", ty: "*mut i32" }, Param { name: "aSimilarItemsInGroup", ty: "*mut i32" }, Param { name: "aPositionInGroup", ty: "*mut i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsIAccessible getChildAtPoint (in long x, in long y); */
                    Method {
                        name: "GetChildAtPoint",
                        params: &[Param { name: "x", ty: "i32" }, Param { name: "y", ty: "i32" }, Param { name: "_retval", ty: "*mut *const nsIAccessible" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsIAccessible getDeepestChildAtPoint (in long x, in long y); */
                    Method {
                        name: "GetDeepestChildAtPoint",
                        params: &[Param { name: "x", ty: "i32" }, Param { name: "y", ty: "i32" }, Param { name: "_retval", ty: "*mut *const nsIAccessible" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsIAccessible getDeepestChildAtPointInProcess (in long x, in long y); */
                    Method {
                        name: "GetDeepestChildAtPointInProcess",
                        params: &[Param { name: "x", ty: "i32" }, Param { name: "y", ty: "i32" }, Param { name: "_retval", ty: "*mut *const nsIAccessible" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsIAccessible getChildAt (in long aChildIndex); */
                    Method {
                        name: "GetChildAt",
                        params: &[Param { name: "aChildIndex", ty: "i32" }, Param { name: "_retval", ty: "*mut *const nsIAccessible" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsIAccessibleRelation getRelationByType (in unsigned long aRelationType); */
                    Method {
                        name: "GetRelationByType",
                        params: &[Param { name: "aRelationType", ty: "u32" }, Param { name: "_retval", ty: "*mut*const nsIAccessibleRelation" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsIArray getRelations (); */
                    Method {
                        name: "GetRelations",
                        params: &[Param { name: "_retval", ty: "*mut *const nsIArray" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void getBounds (out long x, out long y, out long width, out long height); */
                    Method {
                        name: "GetBounds",
                        params: &[Param { name: "x", ty: "*mut i32" }, Param { name: "y", ty: "*mut i32" }, Param { name: "width", ty: "*mut i32" }, Param { name: "height", ty: "*mut i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void getBoundsInCSSPixels (out long aX, out long aY, out long aWidth, out long aHeight); */
                    Method {
                        name: "GetBoundsInCSSPixels",
                        params: &[Param { name: "aX", ty: "*mut i32" }, Param { name: "aY", ty: "*mut i32" }, Param { name: "aWidth", ty: "*mut i32" }, Param { name: "aHeight", ty: "*mut i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void setSelected (in boolean isSelected); */
                    Method {
                        name: "SetSelected",
                        params: &[Param { name: "isSelected", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void takeSelection (); */
                    Method {
                        name: "TakeSelection",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* void takeFocus (); */
                    Method {
                        name: "TakeFocus",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute uint8_t actionCount; */
                    Method {
                        name: "GetActionCount",
                        params: &[Param { name: "aActionCount", ty: "*mut uint8_t" }],
                        ret: "::nserror::nsresult",
                    },

                    /* AString getActionName (in uint8_t index); */
                    Method {
                        name: "GetActionName",
                        params: &[Param { name: "index", ty: "uint8_t" }, Param { name: "_retval", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* AString getActionDescription (in uint8_t aIndex); */
                    Method {
                        name: "GetActionDescription",
                        params: &[Param { name: "aIndex", ty: "uint8_t" }, Param { name: "_retval", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void doAction (in uint8_t index); */
                    Method {
                        name: "DoAction",
                        params: &[Param { name: "index", ty: "uint8_t" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [can_run_script] void scrollTo (in unsigned long aScrollType); */
                    Method {
                        name: "ScrollTo",
                        params: &[Param { name: "aScrollType", ty: "u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void scrollToPoint (in unsigned long coordinateType, in long x, in long y); */
                    Method {
                        name: "ScrollToPoint",
                        params: &[Param { name: "coordinateType", ty: "u32" }, Param { name: "x", ty: "i32" }, Param { name: "y", ty: "i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void announce (in AString announcement, in unsigned short priority); */
                    Method {
                        name: "Announce",
                        params: &[Param { name: "announcement", ty: "*const ::nsstring::nsAString" }, Param { name: "priority", ty: "u16" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

