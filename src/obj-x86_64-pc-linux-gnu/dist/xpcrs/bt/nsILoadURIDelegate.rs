//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/docshell/base/nsILoadURIDelegate.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsILoadURIDelegate",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* boolean loadURI (in nsIURI aURI, in short aWhere, in long aFlags, in nsIPrincipal aTriggeringPrincipal); */
                    Method {
                        name: "LoadURI",
                        params: &[Param { name: "aURI", ty: "*const nsIURI" }, Param { name: "aWhere", ty: "i16" }, Param { name: "aFlags", ty: "i32" }, Param { name: "aTriggeringPrincipal", ty: "*const nsIPrincipal" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsIURI handleLoadError (in nsIURI aURI, in nsresult aError, in short aErrorModule); */
                    Method {
                        name: "HandleLoadError",
                        params: &[Param { name: "aURI", ty: "*const nsIURI" }, Param { name: "aError", ty: "::nserror::nsresult" }, Param { name: "aErrorModule", ty: "i16" }, Param { name: "_retval", ty: "*mut*const nsIURI" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

