/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpfe/appshell/nsIWindowlessBrowser.idl
 */

#ifndef __gen_nsIWindowlessBrowser_h__
#define __gen_nsIWindowlessBrowser_h__


#ifndef __gen_nsIWebNavigation_h__
#include "nsIWebNavigation.h"
#endif

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIDocShell; /* forward declaration */

namespace mozilla {
namespace dom {
class BrowsingContext; /* webidl BrowsingContext */
} // namespace dom
} // namespace mozilla


/* starting interface:    nsIWindowlessBrowser */
#define NS_IWINDOWLESSBROWSER_IID_STR "abb46f48-abfc-41bf-aa9a-7feccefcf977"

#define NS_IWINDOWLESSBROWSER_IID \
  {0xabb46f48, 0xabfc, 0x41bf, \
    { 0xaa, 0x9a, 0x7f, 0xec, 0xce, 0xfc, 0xf9, 0x77 }}

class NS_NO_VTABLE nsIWindowlessBrowser : public nsIWebNavigation {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IWINDOWLESSBROWSER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIWindowlessBrowser;

  /* void close (); */
  NS_IMETHOD Close(void) = 0;

  /* readonly attribute nsIDocShell docShell; */
  NS_IMETHOD GetDocShell(nsIDocShell **aDocShell) = 0;

  /* readonly attribute BrowsingContext browsingContext; */
  NS_IMETHOD GetBrowsingContext(mozilla::dom::BrowsingContext **aBrowsingContext) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIWindowlessBrowser, NS_IWINDOWLESSBROWSER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIWINDOWLESSBROWSER \
  NS_IMETHOD Close(void) override; \
  NS_IMETHOD GetDocShell(nsIDocShell **aDocShell) override; \
  NS_IMETHOD GetBrowsingContext(mozilla::dom::BrowsingContext **aBrowsingContext) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIWINDOWLESSBROWSER \
  nsresult Close(void); \
  nsresult GetDocShell(nsIDocShell **aDocShell); \
  nsresult GetBrowsingContext(mozilla::dom::BrowsingContext **aBrowsingContext); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIWINDOWLESSBROWSER(_to) \
  NS_IMETHOD Close(void) override { return _to Close(); } \
  NS_IMETHOD GetDocShell(nsIDocShell **aDocShell) override { return _to GetDocShell(aDocShell); } \
  NS_IMETHOD GetBrowsingContext(mozilla::dom::BrowsingContext **aBrowsingContext) override { return _to GetBrowsingContext(aBrowsingContext); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIWINDOWLESSBROWSER(_to) \
  NS_IMETHOD Close(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Close(); } \
  NS_IMETHOD GetDocShell(nsIDocShell **aDocShell) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDocShell(aDocShell); } \
  NS_IMETHOD GetBrowsingContext(mozilla::dom::BrowsingContext **aBrowsingContext) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetBrowsingContext(aBrowsingContext); } 


#endif /* __gen_nsIWindowlessBrowser_h__ */
