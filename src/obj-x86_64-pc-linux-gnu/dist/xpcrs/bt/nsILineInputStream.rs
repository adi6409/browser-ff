//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/io/nsILineInputStream.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsILineInputStream",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* boolean readLine (out ACString aLine); */
                    Method {
                        name: "ReadLine",
                        params: &[Param { name: "aLine", ty: "*mut ::nsstring::nsACString" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

