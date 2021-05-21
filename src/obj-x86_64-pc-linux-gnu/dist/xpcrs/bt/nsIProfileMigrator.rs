//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/profile/nsIProfileMigrator.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIProfileStartup",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute nsIFile directory; */
                    Method {
                        name: "GetDirectory",
                        params: &[Param { name: "aDirectory", ty: "*mut*const nsIFile" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void doStartup (); */
                    Method {
                        name: "DoStartup",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIProfileMigrator",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void migrate (in nsIProfileStartup aStartup, in ACString aKey, [optional] in AUTF8String aProfileName); */
                    Method {
                        name: "Migrate",
                        params: &[Param { name: "aStartup", ty: "*const nsIProfileStartup" }, Param { name: "aKey", ty: "*const ::nsstring::nsACString" }, Param { name: "aProfileName", ty: "*const ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

