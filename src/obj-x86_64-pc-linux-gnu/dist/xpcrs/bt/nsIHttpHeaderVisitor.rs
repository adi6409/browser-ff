//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/protocol/http/nsIHttpHeaderVisitor.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIHttpHeaderVisitor",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* [must_use] void visitHeader (in ACString aHeader, in ACString aValue); */
                    Method {
                        name: "VisitHeader",
                        params: &[Param { name: "aHeader", ty: "*const ::nsstring::nsACString" }, Param { name: "aValue", ty: "*const ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

