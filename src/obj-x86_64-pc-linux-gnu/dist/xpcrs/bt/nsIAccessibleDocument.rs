//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/accessible/interfaces/nsIAccessibleDocument.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIAccessibleDocument",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute AString URL; */
                    Method {
                        name: "GetURL",
                        params: &[Param { name: "aURL", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute AString title; */
                    Method {
                        name: "GetTitle",
                        params: &[Param { name: "aTitle", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute AString mimeType; */
                    Method {
                        name: "GetMimeType",
                        params: &[Param { name: "aMimeType", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute AString docType; */
                    Method {
                        name: "GetDocType",
                        params: &[Param { name: "aDocType", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute Document DOMDocument; */
                    Method {
                        name: "GetDOMDocument",
                        params: &[Param { name: "aDOMDocument", ty: "*mut *const libc::c_void" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute mozIDOMWindowProxy window; */
                    Method {
                        name: "GetWindow",
                        params: &[Param { name: "aWindow", ty: "*mut*const mozIDOMWindowProxy" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsIAccessibleDocument parentDocument; */
                    Method {
                        name: "GetParentDocument",
                        params: &[Param { name: "aParentDocument", ty: "*mut *const nsIAccessibleDocument" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute unsigned long childDocumentCount; */
                    Method {
                        name: "GetChildDocumentCount",
                        params: &[Param { name: "aChildDocumentCount", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsIAccessiblePivot virtualCursor; */
                    Method {
                        name: "GetVirtualCursor",
                        params: &[Param { name: "aVirtualCursor", ty: "*mut*const nsIAccessiblePivot" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsIAccessibleDocument getChildDocumentAt (in unsigned long index); */
                    Method {
                        name: "GetChildDocumentAt",
                        params: &[Param { name: "index", ty: "u32" }, Param { name: "_retval", ty: "*mut *const nsIAccessibleDocument" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

