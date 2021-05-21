//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/docshell/base/nsIContentViewerEdit.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIContentViewerEdit",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void clearSelection (); */
                    Method {
                        name: "ClearSelection",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* void selectAll (); */
                    Method {
                        name: "SelectAll",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* void copySelection (); */
                    Method {
                        name: "CopySelection",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute boolean copyable; */
                    Method {
                        name: "GetCopyable",
                        params: &[Param { name: "aCopyable", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void copyLinkLocation (); */
                    Method {
                        name: "CopyLinkLocation",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute boolean inLink; */
                    Method {
                        name: "GetInLink",
                        params: &[Param { name: "aInLink", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void copyImage (in long aCopyFlags); */
                    Method {
                        name: "CopyImage",
                        params: &[Param { name: "aCopyFlags", ty: "i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute boolean inImage; */
                    Method {
                        name: "GetInImage",
                        params: &[Param { name: "aInImage", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* AString getContents (in string aMimeType, in boolean aSelectionOnly); */
                    Method {
                        name: "GetContents",
                        params: &[Param { name: "aMimeType", ty: "*const libc::c_char" }, Param { name: "aSelectionOnly", ty: "bool" }, Param { name: "_retval", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute boolean canGetContents; */
                    Method {
                        name: "GetCanGetContents",
                        params: &[Param { name: "aCanGetContents", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void setCommandNode (in Node aNode); */
                    Method {
                        name: "SetCommandNode",
                        params: &[Param { name: "aNode", ty: "*const libc::c_void" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

