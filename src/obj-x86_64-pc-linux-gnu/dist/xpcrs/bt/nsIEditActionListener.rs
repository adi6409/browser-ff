//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/editor/nsIEditActionListener.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIEditActionListener",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void DidDeleteNode (in Node aChild, in nsresult aResult); */
                    Method {
                        name: "DidDeleteNode",
                        params: &[Param { name: "aChild", ty: "*const libc::c_void" }, Param { name: "aResult", ty: "::nserror::nsresult" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void DidJoinNodes (in Node aLeftNode, in Node aRightNode, in Node aParent, in nsresult aResult); */
                    Method {
                        name: "DidJoinNodes",
                        params: &[Param { name: "aLeftNode", ty: "*const libc::c_void" }, Param { name: "aRightNode", ty: "*const libc::c_void" }, Param { name: "aParent", ty: "*const libc::c_void" }, Param { name: "aResult", ty: "::nserror::nsresult" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void DidInsertText (in CharacterData aTextNode, in long aOffset, in AString aString, in nsresult aResult); */
                    Method {
                        name: "DidInsertText",
                        params: &[Param { name: "aTextNode", ty: "*const libc::c_void" }, Param { name: "aOffset", ty: "i32" }, Param { name: "aString", ty: "*const ::nsstring::nsAString" }, Param { name: "aResult", ty: "::nserror::nsresult" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void WillDeleteText (in CharacterData aTextNode, in long aOffset, in long aLength); */
                    Method {
                        name: "WillDeleteText",
                        params: &[Param { name: "aTextNode", ty: "*const libc::c_void" }, Param { name: "aOffset", ty: "i32" }, Param { name: "aLength", ty: "i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void WillDeleteRanges (in Array<Range> aRangesToDelete); */
                    Method {
                        name: "WillDeleteRanges",
                        params: &[Param { name: "aRangesToDelete", ty: "*const thin_vec::ThinVec<*const libc::c_void>" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

