//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/modules/libjar/nsIJARChannel.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIJARChannel",
            base: Some("nsIChannel"),
            methods: Ok(&[
                    /* attribute nsIFile jarFile; */
                    Method {
                        name: "GetJarFile",
                        params: &[Param { name: "aJarFile", ty: "*mut*const nsIFile" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetJarFile",
                        params: &[Param { name: "aJarFile", ty: "*const nsIFile" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsIZipEntry zipEntry; */
                    Method {
                        name: "GetZipEntry",
                        params: &[Param { name: "aZipEntry", ty: "*mut*const nsIZipEntry" }],
                        ret: "::nserror::nsresult",
                    },

                    /* boolean ensureCached (); */
                    Method {
                        name: "EnsureCached",
                        params: &[Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

