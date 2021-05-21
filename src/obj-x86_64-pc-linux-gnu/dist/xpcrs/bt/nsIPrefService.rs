//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/modules/libpref/nsIPrefService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIPrefStatsCallback",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void visit (in ACString prefName, in unsigned long accessCount); */
                    Method {
                        name: "Visit",
                        params: &[Param { name: "prefName", ty: "*const ::nsstring::nsACString" }, Param { name: "accessCount", ty: "u32" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIPrefService",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void resetPrefs (); */
                    Method {
                        name: "ResetPrefs",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* void resetUserPrefs (); */
                    Method {
                        name: "ResetUserPrefs",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* void savePrefFile (in nsIFile aFile); */
                    Method {
                        name: "SavePrefFile",
                        params: &[Param { name: "aFile", ty: "*const nsIFile" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsIPrefBranch getBranch (in string aPrefRoot); */
                    Method {
                        name: "GetBranch",
                        params: &[Param { name: "aPrefRoot", ty: "*const libc::c_char" }, Param { name: "_retval", ty: "*mut *const nsIPrefBranch" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsIPrefBranch getDefaultBranch (in string aPrefRoot); */
                    Method {
                        name: "GetDefaultBranch",
                        params: &[Param { name: "aPrefRoot", ty: "*const libc::c_char" }, Param { name: "_retval", ty: "*mut *const nsIPrefBranch" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute boolean dirty; */
                    Method {
                        name: "GetDirty",
                        params: &[Param { name: "aDirty", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void readDefaultPrefsFromFile (in nsIFile aFile); */
                    Method {
                        name: "ReadDefaultPrefsFromFile",
                        params: &[Param { name: "aFile", ty: "*const nsIFile" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void readUserPrefsFromFile (in nsIFile aFile); */
                    Method {
                        name: "ReadUserPrefsFromFile",
                        params: &[Param { name: "aFile", ty: "*const nsIFile" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void readStats (in nsIPrefStatsCallback callback); */
                    Method {
                        name: "ReadStats",
                        params: &[Param { name: "callback", ty: "*const nsIPrefStatsCallback" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void resetStats (); */
                    Method {
                        name: "ResetStats",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

