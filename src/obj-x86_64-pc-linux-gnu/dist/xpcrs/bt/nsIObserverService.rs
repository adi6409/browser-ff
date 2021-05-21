//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/ds/nsIObserverService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIObserverService",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void addObserver (in nsIObserver anObserver, in string aTopic, [optional] in boolean ownsWeak); */
                    Method {
                        name: "AddObserver",
                        params: &[Param { name: "anObserver", ty: "*const nsIObserver" }, Param { name: "aTopic", ty: "*const libc::c_char" }, Param { name: "ownsWeak", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void removeObserver (in nsIObserver anObserver, in string aTopic); */
                    Method {
                        name: "RemoveObserver",
                        params: &[Param { name: "anObserver", ty: "*const nsIObserver" }, Param { name: "aTopic", ty: "*const libc::c_char" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void notifyObservers (in nsISupports aSubject, in string aTopic, [optional] in wstring someData); */
                    Method {
                        name: "NotifyObservers",
                        params: &[Param { name: "aSubject", ty: "*const nsISupports" }, Param { name: "aTopic", ty: "*const libc::c_char" }, Param { name: "someData", ty: "*const i16" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsISimpleEnumerator enumerateObservers (in string aTopic); */
                    Method {
                        name: "EnumerateObservers",
                        params: &[Param { name: "aTopic", ty: "*const libc::c_char" }, Param { name: "_retval", ty: "*mut*const nsISimpleEnumerator" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

