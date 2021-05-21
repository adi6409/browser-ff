//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/security/manager/ssl/nsIPKCS11ModuleDB.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIPKCS11ModuleDB",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* [must_use] void deleteModule (in AString moduleName); */
                    Method {
                        name: "DeleteModule",
                        params: &[Param { name: "moduleName", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] void addModule (in AString moduleName, in AString libraryFullPath, in long cryptoMechanismFlags, in long cipherFlags); */
                    Method {
                        name: "AddModule",
                        params: &[Param { name: "moduleName", ty: "*const ::nsstring::nsAString" }, Param { name: "libraryFullPath", ty: "*const ::nsstring::nsAString" }, Param { name: "cryptoMechanismFlags", ty: "i32" }, Param { name: "cipherFlags", ty: "i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] nsISimpleEnumerator listModules (); */
                    Method {
                        name: "ListModules",
                        params: &[Param { name: "_retval", ty: "*mut*const nsISimpleEnumerator" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] readonly attribute boolean canToggleFIPS; */
                    Method {
                        name: "GetCanToggleFIPS",
                        params: &[Param { name: "aCanToggleFIPS", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] void toggleFIPSMode (); */
                    Method {
                        name: "ToggleFIPSMode",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] readonly attribute boolean isFIPSEnabled; */
                    Method {
                        name: "GetIsFIPSEnabled",
                        params: &[Param { name: "aIsFIPSEnabled", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

