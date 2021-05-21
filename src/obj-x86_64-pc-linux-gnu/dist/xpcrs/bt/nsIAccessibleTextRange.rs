//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/accessible/interfaces/nsIAccessibleTextRange.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIAccessibleTextRange",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute nsIAccessibleText startContainer; */
                    Method {
                        name: "GetStartContainer",
                        params: &[Param { name: "aStartContainer", ty: "*mut*const nsIAccessibleText" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute long startOffset; */
                    Method {
                        name: "GetStartOffset",
                        params: &[Param { name: "aStartOffset", ty: "*mut i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsIAccessibleText endContainer; */
                    Method {
                        name: "GetEndContainer",
                        params: &[Param { name: "aEndContainer", ty: "*mut*const nsIAccessibleText" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute long endOffset; */
                    Method {
                        name: "GetEndOffset",
                        params: &[Param { name: "aEndOffset", ty: "*mut i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsIAccessible container; */
                    Method {
                        name: "GetContainer",
                        params: &[Param { name: "aContainer", ty: "*mut*const nsIAccessible" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsIArray embeddedChildren; */
                    Method {
                        name: "GetEmbeddedChildren",
                        params: &[Param { name: "aEmbeddedChildren", ty: "*mut*const nsIArray" }],
                        ret: "::nserror::nsresult",
                    },

                    /* boolean compare (in nsIAccessibleTextRange aOtherRange); */
                    Method {
                        name: "Compare",
                        params: &[Param { name: "aOtherRange", ty: "*const nsIAccessibleTextRange" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* long compareEndPoints (in unsigned long aEndPoint, in nsIAccessibleTextRange aOtherRange, in unsigned long aOtherRangeEndPoint); */
                    Method {
                        name: "CompareEndPoints",
                        params: &[Param { name: "aEndPoint", ty: "u32" }, Param { name: "aOtherRange", ty: "*const nsIAccessibleTextRange" }, Param { name: "aOtherRangeEndPoint", ty: "u32" }, Param { name: "_retval", ty: "*mut i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute AString text; */
                    Method {
                        name: "GetText",
                        params: &[Param { name: "aText", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsIArray bounds; */
                    Method {
                        name: "GetBounds",
                        params: &[Param { name: "aBounds", ty: "*mut*const nsIArray" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void move (in unsigned long aUnit, in long aCount); */
                    Method {
                        name: "Move",
                        params: &[Param { name: "aUnit", ty: "u32" }, Param { name: "aCount", ty: "i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void moveStart (in unsigned long aUnit, in long aCount); */
                    Method {
                        name: "MoveStart",
                        params: &[Param { name: "aUnit", ty: "u32" }, Param { name: "aCount", ty: "i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void moveEnd (in unsigned long aUnit, in long aCount); */
                    Method {
                        name: "MoveEnd",
                        params: &[Param { name: "aUnit", ty: "u32" }, Param { name: "aCount", ty: "i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void normalize (in unsigned long aUnit); */
                    Method {
                        name: "Normalize",
                        params: &[Param { name: "aUnit", ty: "u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* boolean crop (in nsIAccessible aContainer); */
                    Method {
                        name: "Crop",
                        params: &[Param { name: "aContainer", ty: "*const nsIAccessible" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsIAccessibleTextRange findText (in AString aText, in boolean aIsBackward, in boolean aIsIgnoreCase); */
                    Method {
                        name: "FindText",
                        params: &[Param { name: "aText", ty: "*const ::nsstring::nsAString" }, Param { name: "aIsBackward", ty: "bool" }, Param { name: "aIsIgnoreCase", ty: "bool" }, Param { name: "_retval", ty: "*mut *const nsIAccessibleTextRange" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsIAccessibleTextRange findAttr (in unsigned long aAttr, in nsIVariant aValue, in boolean aIsBackward); */
                    Method {
                        name: "FindAttr",
                        params: &[Param { name: "aAttr", ty: "u32" }, Param { name: "aValue", ty: "*const nsIVariant" }, Param { name: "aIsBackward", ty: "bool" }, Param { name: "_retval", ty: "*mut *const nsIAccessibleTextRange" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void addToSelection (); */
                    Method {
                        name: "AddToSelection",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* void removeFromSelection (); */
                    Method {
                        name: "RemoveFromSelection",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* void select (); */
                    Method {
                        name: "Select",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* void scrollIntoView (in unsigned long aHow); */
                    Method {
                        name: "ScrollIntoView",
                        params: &[Param { name: "aHow", ty: "u32" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

