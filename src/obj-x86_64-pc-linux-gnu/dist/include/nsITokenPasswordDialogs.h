/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/security/manager/ssl/nsITokenPasswordDialogs.idl
 */

#ifndef __gen_nsITokenPasswordDialogs_h__
#define __gen_nsITokenPasswordDialogs_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIInterfaceRequestor; /* forward declaration */

class nsIPK11Token; /* forward declaration */


/* starting interface:    nsITokenPasswordDialogs */
#define NS_ITOKENPASSWORDDIALOGS_IID_STR "87dbd64a-4466-474e-95f5-1ad1cee5702c"

#define NS_ITOKENPASSWORDDIALOGS_IID \
  {0x87dbd64a, 0x4466, 0x474e, \
    { 0x95, 0xf5, 0x1a, 0xd1, 0xce, 0xe5, 0x70, 0x2c }}

class NS_NO_VTABLE nsITokenPasswordDialogs : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ITOKENPASSWORDDIALOGS_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsITokenPasswordDialogs;

  /* [must_use] boolean setPassword (in nsIInterfaceRequestor ctx, in nsIPK11Token token); */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD SetPassword(nsIInterfaceRequestor *ctx, nsIPK11Token *token, bool *_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsITokenPasswordDialogs, NS_ITOKENPASSWORDDIALOGS_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSITOKENPASSWORDDIALOGS \
  [[nodiscard]] NS_IMETHOD SetPassword(nsIInterfaceRequestor *ctx, nsIPK11Token *token, bool *_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSITOKENPASSWORDDIALOGS \
  [[nodiscard]] nsresult SetPassword(nsIInterfaceRequestor *ctx, nsIPK11Token *token, bool *_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSITOKENPASSWORDDIALOGS(_to) \
  [[nodiscard]] NS_IMETHOD SetPassword(nsIInterfaceRequestor *ctx, nsIPK11Token *token, bool *_retval) override { return _to SetPassword(ctx, token, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSITOKENPASSWORDDIALOGS(_to) \
  [[nodiscard]] NS_IMETHOD SetPassword(nsIInterfaceRequestor *ctx, nsIPK11Token *token, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetPassword(ctx, token, _retval); } 

#define NS_TOKENPASSWORDSDIALOG_CONTRACTID "@mozilla.org/nsTokenPasswordDialogs;1"

#endif /* __gen_nsITokenPasswordDialogs_h__ */
