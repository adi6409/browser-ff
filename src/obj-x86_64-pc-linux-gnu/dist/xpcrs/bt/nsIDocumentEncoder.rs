//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/serializers/nsIDocumentEncoder.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDocumentEncoderNodeFixup",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* Node fixupNode (in Node aNode, out boolean aSerializeCloneKids); */
                    Method {
                        name: "FixupNode",
                        params: &[Param { name: "aNode", ty: "*const libc::c_void" }, Param { name: "aSerializeCloneKids", ty: "*mut bool" }, Param { name: "_retval", ty: "*mut *const libc::c_void" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIDocumentEncoder",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void init (in Document aDocument, in AString aMimeType, in unsigned long aFlags); */
                    Method {
                        name: "Init",
                        params: &[Param { name: "aDocument", ty: "*const libc::c_void" }, Param { name: "aMimeType", ty: "*const ::nsstring::nsAString" }, Param { name: "aFlags", ty: "u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [noscript] void nativeInit (in Document aDocument, in AString aMimeType, in unsigned long aFlags); */
                    Method {
                        name: "NativeInit",
                        params: &[Param { name: "aDocument", ty: "*const libc::c_void" }, Param { name: "aMimeType", ty: "*const ::nsstring::nsAString" }, Param { name: "aFlags", ty: "u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void setSelection (in Selection aSelection); */
                    Method {
                        name: "SetSelection",
                        params: &[Param { name: "aSelection", ty: "*const libc::c_void" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void setRange (in Range aRange); */
                    Method {
                        name: "SetRange",
                        params: &[Param { name: "aRange", ty: "*const libc::c_void" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void setNode (in Node aNode); */
                    Method {
                        name: "SetNode",
                        params: &[Param { name: "aNode", ty: "*const libc::c_void" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void setContainerNode (in Node aContainer); */
                    Method {
                        name: "SetContainerNode",
                        params: &[Param { name: "aContainer", ty: "*const libc::c_void" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void setCharset (in ACString aCharset); */
                    Method {
                        name: "SetCharset",
                        params: &[Param { name: "aCharset", ty: "*const ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void setWrapColumn (in unsigned long aWrapColumn); */
                    Method {
                        name: "SetWrapColumn",
                        params: &[Param { name: "aWrapColumn", ty: "u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute AString mimeType; */
                    Method {
                        name: "GetMimeType",
                        params: &[Param { name: "aMimeType", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void encodeToStream (in nsIOutputStream aStream); */
                    Method {
                        name: "EncodeToStream",
                        params: &[Param { name: "aStream", ty: "*const nsIOutputStream" }],
                        ret: "::nserror::nsresult",
                    },

                    /* AString encodeToString (); */
                    Method {
                        name: "EncodeToString",
                        params: &[Param { name: "_retval", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* AString encodeToStringWithContext (out AString aContextString, out AString aInfoString); */
                    Method {
                        name: "EncodeToStringWithContext",
                        params: &[Param { name: "aContextString", ty: "*mut ::nsstring::nsAString" }, Param { name: "aInfoString", ty: "*mut ::nsstring::nsAString" }, Param { name: "_retval", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* AString encodeToStringWithMaxLength (in unsigned long aMaxLength); */
                    Method {
                        name: "EncodeToStringWithMaxLength",
                        params: &[Param { name: "aMaxLength", ty: "u32" }, Param { name: "_retval", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void setNodeFixup (in nsIDocumentEncoderNodeFixup aFixup); */
                    Method {
                        name: "SetNodeFixup",
                        params: &[Param { name: "aFixup", ty: "*const nsIDocumentEncoderNodeFixup" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

