//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIMIMEInputStream.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIMIMEInputStream",
            base: Some("nsIInputStream"),
            methods: Ok(&[
                    /* void addHeader (in string name, in string value); */
                    Method {
                        name: "AddHeader",
                        params: &[Param { name: "name", ty: "*const libc::c_char" }, Param { name: "value", ty: "*const libc::c_char" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void visitHeaders (in nsIHttpHeaderVisitor visitor); */
                    Method {
                        name: "VisitHeaders",
                        params: &[Param { name: "visitor", ty: "*const nsIHttpHeaderVisitor" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void setData (in nsIInputStream stream); */
                    Method {
                        name: "SetData",
                        params: &[Param { name: "stream", ty: "*const nsIInputStream" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsIInputStream data; */
                    Method {
                        name: "GetData",
                        params: &[Param { name: "aData", ty: "*mut *const nsIInputStream" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

