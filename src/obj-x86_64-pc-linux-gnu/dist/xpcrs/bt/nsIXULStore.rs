//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/xulstore/nsIXULStore.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIXULStore",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void persist (in Node aNode, in AString attr); */
                    Method {
                        name: "Persist",
                        params: &[Param { name: "aNode", ty: "*const libc::c_void" }, Param { name: "attr", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void setValue (in AString doc, in AString id, in AString attr, in AString value); */
                    Method {
                        name: "SetValue",
                        params: &[Param { name: "doc", ty: "*const ::nsstring::nsAString" }, Param { name: "id", ty: "*const ::nsstring::nsAString" }, Param { name: "attr", ty: "*const ::nsstring::nsAString" }, Param { name: "value", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* bool hasValue (in AString doc, in AString id, in AString attr); */
                    Method {
                        name: "HasValue",
                        params: &[Param { name: "doc", ty: "*const ::nsstring::nsAString" }, Param { name: "id", ty: "*const ::nsstring::nsAString" }, Param { name: "attr", ty: "*const ::nsstring::nsAString" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* AString getValue (in AString doc, in AString id, in AString attr); */
                    Method {
                        name: "GetValue",
                        params: &[Param { name: "doc", ty: "*const ::nsstring::nsAString" }, Param { name: "id", ty: "*const ::nsstring::nsAString" }, Param { name: "attr", ty: "*const ::nsstring::nsAString" }, Param { name: "_retval", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void removeValue (in AString doc, in AString id, in AString attr); */
                    Method {
                        name: "RemoveValue",
                        params: &[Param { name: "doc", ty: "*const ::nsstring::nsAString" }, Param { name: "id", ty: "*const ::nsstring::nsAString" }, Param { name: "attr", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void removeDocument (in AString doc); */
                    Method {
                        name: "RemoveDocument",
                        params: &[Param { name: "doc", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsIStringEnumerator getIDsEnumerator (in AString doc); */
                    Method {
                        name: "GetIDsEnumerator",
                        params: &[Param { name: "doc", ty: "*const ::nsstring::nsAString" }, Param { name: "_retval", ty: "*mut*const nsIStringEnumerator" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsIStringEnumerator getAttributeEnumerator (in AString doc, in AString id); */
                    Method {
                        name: "GetAttributeEnumerator",
                        params: &[Param { name: "doc", ty: "*const ::nsstring::nsAString" }, Param { name: "id", ty: "*const ::nsstring::nsAString" }, Param { name: "_retval", ty: "*mut*const nsIStringEnumerator" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

