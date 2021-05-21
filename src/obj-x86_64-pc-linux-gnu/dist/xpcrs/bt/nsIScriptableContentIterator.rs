//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/base/nsIScriptableContentIterator.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIScriptableContentIterator",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void initWithRootNode (in nsIScriptableContentIterator_IteratorType aType, in Node aRoot); */
                    Method {
                        name: "InitWithRootNode",
                        params: &[Param { name: "aType", ty: " u8" }, Param { name: "aRoot", ty: "*const libc::c_void" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void initWithRange (in nsIScriptableContentIterator_IteratorType aType, in Range aRange); */
                    Method {
                        name: "InitWithRange",
                        params: &[Param { name: "aType", ty: " u8" }, Param { name: "aRange", ty: "*const libc::c_void" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void initWithPositions (in nsIScriptableContentIterator_IteratorType aType, in Node aStartContainer, in unsigned long aStartOffset, in Node aEndContainer, in unsigned long aEndOffset); */
                    Method {
                        name: "InitWithPositions",
                        params: &[Param { name: "aType", ty: " u8" }, Param { name: "aStartContainer", ty: "*const libc::c_void" }, Param { name: "aStartOffset", ty: "u32" }, Param { name: "aEndContainer", ty: "*const libc::c_void" }, Param { name: "aEndOffset", ty: "u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void first (); */
                    Method {
                        name: "First",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* void last (); */
                    Method {
                        name: "Last",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* void next (); */
                    Method {
                        name: "Next",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* void prev (); */
                    Method {
                        name: "Prev",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute Node currentNode; */
                    Method {
                        name: "GetCurrentNode",
                        params: &[Param { name: "aCurrentNode", ty: "*mut *const libc::c_void" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute bool isDone; */
                    Method {
                        name: "GetIsDone",
                        params: &[Param { name: "aIsDone", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void positionAt (in Node aNode); */
                    Method {
                        name: "PositionAt",
                        params: &[Param { name: "aNode", ty: "*const libc::c_void" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

