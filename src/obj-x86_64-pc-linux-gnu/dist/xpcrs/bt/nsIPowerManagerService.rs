//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/power/nsIPowerManagerService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIPowerManagerService",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void addWakeLockListener (in nsIDOMMozWakeLockListener aListener); */
                    Method {
                        name: "AddWakeLockListener",
                        params: &[Param { name: "aListener", ty: "*const nsIDOMMozWakeLockListener" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void removeWakeLockListener (in nsIDOMMozWakeLockListener aListener); */
                    Method {
                        name: "RemoveWakeLockListener",
                        params: &[Param { name: "aListener", ty: "*const nsIDOMMozWakeLockListener" }],
                        ret: "::nserror::nsresult",
                    },

                    /* AString getWakeLockState (in AString aTopic); */
                    Method {
                        name: "GetWakeLockState",
                        params: &[Param { name: "aTopic", ty: "*const ::nsstring::nsAString" }, Param { name: "_retval", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsIWakeLock newWakeLock (in AString aTopic, [optional] in mozIDOMWindow aWindow); */
                    Method {
                        name: "NewWakeLock",
                        params: &[Param { name: "aTopic", ty: "*const ::nsstring::nsAString" }, Param { name: "aWindow", ty: "*const mozIDOMWindow" }, Param { name: "_retval", ty: "*mut*const nsIWakeLock" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

