/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/uriloader/exthandler/nsIContentDispatchChooser.idl
 */

#ifndef __gen_nsIContentDispatchChooser_h__
#define __gen_nsIContentDispatchChooser_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIHandlerInfo; /* forward declaration */

class nsIPrincipal; /* forward declaration */

class nsIURI; /* forward declaration */

namespace mozilla {
namespace dom {
class BrowsingContext; /* webidl BrowsingContext */
} // namespace dom
} // namespace mozilla


/* starting interface:    nsIContentDispatchChooser */
#define NS_ICONTENTDISPATCHCHOOSER_IID_STR "456ca3b2-02be-4f97-89a2-08c08d3ad88f"

#define NS_ICONTENTDISPATCHCHOOSER_IID \
  {0x456ca3b2, 0x02be, 0x4f97, \
    { 0x89, 0xa2, 0x08, 0xc0, 0x8d, 0x3a, 0xd8, 0x8f }}

class NS_NO_VTABLE nsIContentDispatchChooser : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ICONTENTDISPATCHCHOOSER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIContentDispatchChooser;

  /* void handleURI (in nsIHandlerInfo aHandler, in nsIURI aURI, in nsIPrincipal aTriggeringPrincipal, in BrowsingContext aBrowsingContext); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD HandleURI(nsIHandlerInfo *aHandler, nsIURI *aURI, nsIPrincipal *aTriggeringPrincipal, mozilla::dom::BrowsingContext *aBrowsingContext) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIContentDispatchChooser, NS_ICONTENTDISPATCHCHOOSER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSICONTENTDISPATCHCHOOSER \
  NS_IMETHOD HandleURI(nsIHandlerInfo *aHandler, nsIURI *aURI, nsIPrincipal *aTriggeringPrincipal, mozilla::dom::BrowsingContext *aBrowsingContext) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSICONTENTDISPATCHCHOOSER \
  nsresult HandleURI(nsIHandlerInfo *aHandler, nsIURI *aURI, nsIPrincipal *aTriggeringPrincipal, mozilla::dom::BrowsingContext *aBrowsingContext); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSICONTENTDISPATCHCHOOSER(_to) \
  NS_IMETHOD HandleURI(nsIHandlerInfo *aHandler, nsIURI *aURI, nsIPrincipal *aTriggeringPrincipal, mozilla::dom::BrowsingContext *aBrowsingContext) override { return _to HandleURI(aHandler, aURI, aTriggeringPrincipal, aBrowsingContext); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSICONTENTDISPATCHCHOOSER(_to) \
  NS_IMETHOD HandleURI(nsIHandlerInfo *aHandler, nsIURI *aURI, nsIPrincipal *aTriggeringPrincipal, mozilla::dom::BrowsingContext *aBrowsingContext) override { return !_to ? NS_ERROR_NULL_POINTER : _to->HandleURI(aHandler, aURI, aTriggeringPrincipal, aBrowsingContext); } 


#endif /* __gen_nsIContentDispatchChooser_h__ */
