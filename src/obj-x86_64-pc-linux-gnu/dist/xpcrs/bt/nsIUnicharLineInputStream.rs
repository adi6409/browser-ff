//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/io/nsIUnicharLineInputStream.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIUnicharLineInputStream",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* boolean readLine (out AString aLine); */
                    Method {
                        name: "ReadLine",
                        params: &[Param { name: "aLine", ty: "*mut ::nsstring::nsAString" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

