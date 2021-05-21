/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/browser/nsIPrintingPromptService.idl
 */

#ifndef __gen_nsIPrintingPromptService_h__
#define __gen_nsIPrintingPromptService_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#ifndef __gen_nsIWebBrowserPrint_h__
#include "nsIWebBrowserPrint.h"
#endif

#ifndef __gen_nsIWebProgressListener_h__
#include "nsIWebProgressListener.h"
#endif

#ifndef __gen_nsIPrintProgressParams_h__
#include "nsIPrintProgressParams.h"
#endif

#ifndef __gen_nsIPrintSettings_h__
#include "nsIPrintSettings.h"
#endif

#ifndef __gen_nsIObserver_h__
#include "nsIObserver.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsIPrintingPromptService */
#define NS_IPRINTINGPROMPTSERVICE_IID_STR "72006d06-a2a5-4250-ae92-04b2f0e2ab8d"

#define NS_IPRINTINGPROMPTSERVICE_IID \
  {0x72006d06, 0xa2a5, 0x4250, \
    { 0xae, 0x92, 0x04, 0xb2, 0xf0, 0xe2, 0xab, 0x8d }}

class NS_NO_VTABLE nsIPrintingPromptService : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IPRINTINGPROMPTSERVICE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIPrintingPromptService;

  /* void showPrintDialog (in mozIDOMWindowProxy parent, in nsIPrintSettings printSettings); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ShowPrintDialog(mozIDOMWindowProxy *parent, nsIPrintSettings *printSettings) = 0;

  /* void showPrintProgressDialog (in mozIDOMWindowProxy parent, in nsIPrintSettings printSettings, in nsIObserver openDialogObserver, in boolean isForPrinting, out nsIWebProgressListener webProgressListener, out nsIPrintProgressParams printProgressParams, out boolean notifyOnOpen); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ShowPrintProgressDialog(mozIDOMWindowProxy *parent, nsIPrintSettings *printSettings, nsIObserver *openDialogObserver, bool isForPrinting, nsIWebProgressListener **webProgressListener, nsIPrintProgressParams **printProgressParams, bool *notifyOnOpen) = 0;

  /* void showPageSetupDialog (in mozIDOMWindowProxy parent, in nsIPrintSettings printSettings); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ShowPageSetupDialog(mozIDOMWindowProxy *parent, nsIPrintSettings *printSettings) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIPrintingPromptService, NS_IPRINTINGPROMPTSERVICE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIPRINTINGPROMPTSERVICE \
  NS_IMETHOD ShowPrintDialog(mozIDOMWindowProxy *parent, nsIPrintSettings *printSettings) override; \
  NS_IMETHOD ShowPrintProgressDialog(mozIDOMWindowProxy *parent, nsIPrintSettings *printSettings, nsIObserver *openDialogObserver, bool isForPrinting, nsIWebProgressListener **webProgressListener, nsIPrintProgressParams **printProgressParams, bool *notifyOnOpen) override; \
  NS_IMETHOD ShowPageSetupDialog(mozIDOMWindowProxy *parent, nsIPrintSettings *printSettings) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIPRINTINGPROMPTSERVICE \
  nsresult ShowPrintDialog(mozIDOMWindowProxy *parent, nsIPrintSettings *printSettings); \
  nsresult ShowPrintProgressDialog(mozIDOMWindowProxy *parent, nsIPrintSettings *printSettings, nsIObserver *openDialogObserver, bool isForPrinting, nsIWebProgressListener **webProgressListener, nsIPrintProgressParams **printProgressParams, bool *notifyOnOpen); \
  nsresult ShowPageSetupDialog(mozIDOMWindowProxy *parent, nsIPrintSettings *printSettings); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIPRINTINGPROMPTSERVICE(_to) \
  NS_IMETHOD ShowPrintDialog(mozIDOMWindowProxy *parent, nsIPrintSettings *printSettings) override { return _to ShowPrintDialog(parent, printSettings); } \
  NS_IMETHOD ShowPrintProgressDialog(mozIDOMWindowProxy *parent, nsIPrintSettings *printSettings, nsIObserver *openDialogObserver, bool isForPrinting, nsIWebProgressListener **webProgressListener, nsIPrintProgressParams **printProgressParams, bool *notifyOnOpen) override { return _to ShowPrintProgressDialog(parent, printSettings, openDialogObserver, isForPrinting, webProgressListener, printProgressParams, notifyOnOpen); } \
  NS_IMETHOD ShowPageSetupDialog(mozIDOMWindowProxy *parent, nsIPrintSettings *printSettings) override { return _to ShowPageSetupDialog(parent, printSettings); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIPRINTINGPROMPTSERVICE(_to) \
  NS_IMETHOD ShowPrintDialog(mozIDOMWindowProxy *parent, nsIPrintSettings *printSettings) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ShowPrintDialog(parent, printSettings); } \
  NS_IMETHOD ShowPrintProgressDialog(mozIDOMWindowProxy *parent, nsIPrintSettings *printSettings, nsIObserver *openDialogObserver, bool isForPrinting, nsIWebProgressListener **webProgressListener, nsIPrintProgressParams **printProgressParams, bool *notifyOnOpen) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ShowPrintProgressDialog(parent, printSettings, openDialogObserver, isForPrinting, webProgressListener, printProgressParams, notifyOnOpen); } \
  NS_IMETHOD ShowPageSetupDialog(mozIDOMWindowProxy *parent, nsIPrintSettings *printSettings) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ShowPageSetupDialog(parent, printSettings); } 

// {260FEDC5-524D-4aa6-9A41-E829F4C78B92}
#define NS_PRINTINGPROMPTSERVICE_IID \
 {0x260fedc5, 0x524d, 0x4aa6, { 0x9a, 0x41, 0xe8, 0x29, 0xf4, 0xc7, 0x8b, 0x92}}

#endif /* __gen_nsIPrintingPromptService_h__ */
