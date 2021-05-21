//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/chrome/nsIToolkitChromeRegistry.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIToolkitChromeRegistry",
            base: Some("nsIXULChromeRegistry"),
            methods: Ok(&[
                    /* nsIUTF8StringEnumerator getLocalesForPackage (in AUTF8String aPackage); */
                    Method {
                        name: "GetLocalesForPackage",
                        params: &[Param { name: "aPackage", ty: "*const ::nsstring::nsACString" }, Param { name: "_retval", ty: "*mut*const nsIUTF8StringEnumerator" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

