//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/accessible/interfaces/nsIAccessiblePivot.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIAccessiblePivot",
            base: Some("nsISupports"),
            methods: Err("optional_argc is unsupported"),
        },

        Interface {
            name: "nsIAccessiblePivotObserver",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void onPivotChanged (in nsIAccessiblePivot aPivot, in nsIAccessible aOldAccessible, in long aOldStart, in long aOldEnd, in nsIAccessible aNewAccessible, in long aNewStart, in long aNewEnd, in PivotMoveReason aReason, in TextBoundaryType aBoundaryType, in boolean aIsFromUserInput); */
                    Method {
                        name: "OnPivotChanged",
                        params: &[Param { name: "aPivot", ty: "*const nsIAccessiblePivot" }, Param { name: "aOldAccessible", ty: "*const nsIAccessible" }, Param { name: "aOldStart", ty: "i32" }, Param { name: "aOldEnd", ty: "i32" }, Param { name: "aNewAccessible", ty: "*const nsIAccessible" }, Param { name: "aNewStart", ty: "i32" }, Param { name: "aNewEnd", ty: "i32" }, Param { name: "aReason", ty: "PivotMoveReason" }, Param { name: "aBoundaryType", ty: "TextBoundaryType" }, Param { name: "aIsFromUserInput", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIAccessibleTraversalRule",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute unsigned long preFilter; */
                    Method {
                        name: "GetPreFilter",
                        params: &[Param { name: "aPreFilter", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* Array<unsigned long> getMatchRoles (); */
                    Method {
                        name: "GetMatchRoles",
                        params: &[Param { name: "_retval", ty: "*mut thin_vec::ThinVec<u32>" }],
                        ret: "::nserror::nsresult",
                    },

                    /* unsigned short match (in nsIAccessible aAccessible); */
                    Method {
                        name: "Match",
                        params: &[Param { name: "aAccessible", ty: "*const nsIAccessible" }, Param { name: "_retval", ty: "*mut u16" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

