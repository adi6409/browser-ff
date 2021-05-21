/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/uriloader/exthandler/nsIHelperAppLauncherDialog.idl
 */

#ifndef __gen_nsIHelperAppLauncherDialog_h__
#define __gen_nsIHelperAppLauncherDialog_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIHelperAppLauncher; /* forward declaration */

class nsIFile; /* forward declaration */

class nsIInterfaceRequestor; /* forward declaration */


/* starting interface:    nsIHelperAppLauncherDialog */
#define NS_IHELPERAPPLAUNCHERDIALOG_IID_STR "bfc739f3-8d75-4034-a6f8-1039a5996bad"

#define NS_IHELPERAPPLAUNCHERDIALOG_IID \
  {0xbfc739f3, 0x8d75, 0x4034, \
    { 0xa6, 0xf8, 0x10, 0x39, 0xa5, 0x99, 0x6b, 0xad }}

class NS_NO_VTABLE nsIHelperAppLauncherDialog : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IHELPERAPPLAUNCHERDIALOG_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIHelperAppLauncherDialog;

  enum {
    REASON_CANTHANDLE = 0U,
    REASON_SERVERREQUEST = 1U,
    REASON_TYPESNIFFED = 2U
  };

  /* void show (in nsIHelperAppLauncher aLauncher, in nsIInterfaceRequestor aWindowContext, in unsigned long aReason); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Show(nsIHelperAppLauncher *aLauncher, nsIInterfaceRequestor *aWindowContext, uint32_t aReason) = 0;

  /* void promptForSaveToFileAsync (in nsIHelperAppLauncher aLauncher, in nsIInterfaceRequestor aWindowContext, in wstring aDefaultFileName, in wstring aSuggestedFileExtension, in boolean aForcePrompt); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD PromptForSaveToFileAsync(nsIHelperAppLauncher *aLauncher, nsIInterfaceRequestor *aWindowContext, const char16_t * aDefaultFileName, const char16_t * aSuggestedFileExtension, bool aForcePrompt) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIHelperAppLauncherDialog, NS_IHELPERAPPLAUNCHERDIALOG_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIHELPERAPPLAUNCHERDIALOG \
  NS_IMETHOD Show(nsIHelperAppLauncher *aLauncher, nsIInterfaceRequestor *aWindowContext, uint32_t aReason) override; \
  NS_IMETHOD PromptForSaveToFileAsync(nsIHelperAppLauncher *aLauncher, nsIInterfaceRequestor *aWindowContext, const char16_t * aDefaultFileName, const char16_t * aSuggestedFileExtension, bool aForcePrompt) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIHELPERAPPLAUNCHERDIALOG \
  nsresult Show(nsIHelperAppLauncher *aLauncher, nsIInterfaceRequestor *aWindowContext, uint32_t aReason); \
  nsresult PromptForSaveToFileAsync(nsIHelperAppLauncher *aLauncher, nsIInterfaceRequestor *aWindowContext, const char16_t * aDefaultFileName, const char16_t * aSuggestedFileExtension, bool aForcePrompt); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIHELPERAPPLAUNCHERDIALOG(_to) \
  NS_IMETHOD Show(nsIHelperAppLauncher *aLauncher, nsIInterfaceRequestor *aWindowContext, uint32_t aReason) override { return _to Show(aLauncher, aWindowContext, aReason); } \
  NS_IMETHOD PromptForSaveToFileAsync(nsIHelperAppLauncher *aLauncher, nsIInterfaceRequestor *aWindowContext, const char16_t * aDefaultFileName, const char16_t * aSuggestedFileExtension, bool aForcePrompt) override { return _to PromptForSaveToFileAsync(aLauncher, aWindowContext, aDefaultFileName, aSuggestedFileExtension, aForcePrompt); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIHELPERAPPLAUNCHERDIALOG(_to) \
  NS_IMETHOD Show(nsIHelperAppLauncher *aLauncher, nsIInterfaceRequestor *aWindowContext, uint32_t aReason) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Show(aLauncher, aWindowContext, aReason); } \
  NS_IMETHOD PromptForSaveToFileAsync(nsIHelperAppLauncher *aLauncher, nsIInterfaceRequestor *aWindowContext, const char16_t * aDefaultFileName, const char16_t * aSuggestedFileExtension, bool aForcePrompt) override { return !_to ? NS_ERROR_NULL_POINTER : _to->PromptForSaveToFileAsync(aLauncher, aWindowContext, aDefaultFileName, aSuggestedFileExtension, aForcePrompt); } 

#define NS_HELPERAPPLAUNCHERDLG_CONTRACTID    "@mozilla.org/helperapplauncherdialog;1"

#endif /* __gen_nsIHelperAppLauncherDialog_h__ */
