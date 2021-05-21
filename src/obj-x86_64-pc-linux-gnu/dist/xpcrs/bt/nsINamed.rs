//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/threads/nsINamed.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsINamed",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute AUTF8String name; */
                    Method {
                        name: "GetName",
                        params: &[Param { name: "aName", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

