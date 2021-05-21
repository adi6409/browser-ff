/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/accessible/interfaces/nsIAccessibilityService.idl
 */

#ifndef __gen_nsIAccessibilityService_h__
#define __gen_nsIAccessibilityService_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIAccessible; /* forward declaration */

class nsIWeakReference; /* forward declaration */

class nsIAccessiblePivot; /* forward declaration */

class nsINode; /* webidl Node */


/* starting interface:    nsIAccessibilityService */
#define NS_IACCESSIBILITYSERVICE_IID_STR "2188e3a0-c88e-11e7-8f1a-0800200c9a66"

#define NS_IACCESSIBILITYSERVICE_IID \
  {0x2188e3a0, 0xc88e, 0x11e7, \
    { 0x8f, 0x1a, 0x08, 0x00, 0x20, 0x0c, 0x9a, 0x66 }}

class NS_NO_VTABLE nsIAccessibilityService : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IACCESSIBILITYSERVICE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIAccessibilityService;

  /* nsIAccessible getApplicationAccessible (); */
  NS_IMETHOD GetApplicationAccessible(nsIAccessible **_retval) = 0;

  /* nsIAccessible getAccessibleFor (in Node aNode); */
  NS_IMETHOD GetAccessibleFor(nsINode *aNode, nsIAccessible **_retval) = 0;

  /* AString getStringRole (in unsigned long aRole); */
  NS_IMETHOD GetStringRole(uint32_t aRole, nsAString& _retval) = 0;

  /* nsISupports getStringStates (in unsigned long aStates, in unsigned long aExtraStates); */
  NS_IMETHOD GetStringStates(uint32_t aStates, uint32_t aExtraStates, nsISupports **_retval) = 0;

  /* AString getStringEventType (in unsigned long aEventType); */
  NS_IMETHOD GetStringEventType(uint32_t aEventType, nsAString& _retval) = 0;

  /* AString getStringRelationType (in unsigned long aRelationType); */
  NS_IMETHOD GetStringRelationType(uint32_t aRelationType, nsAString& _retval) = 0;

  /* nsIAccessible getAccessibleFromCache (in Node aNode); */
  NS_IMETHOD GetAccessibleFromCache(nsINode *aNode, nsIAccessible **_retval) = 0;

  /* nsIAccessiblePivot createAccessiblePivot (in nsIAccessible aRoot); */
  NS_IMETHOD CreateAccessiblePivot(nsIAccessible *aRoot, nsIAccessiblePivot **_retval) = 0;

  /* void setLogging (in ACString aModules); */
  NS_IMETHOD SetLogging(const nsACString& aModules) = 0;

  /* boolean isLogged (in AString aModule); */
  NS_IMETHOD IsLogged(const nsAString& aModule, bool *_retval) = 0;

  /* AString getConsumers (); */
  NS_IMETHOD GetConsumers(nsAString& _retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIAccessibilityService, NS_IACCESSIBILITYSERVICE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIACCESSIBILITYSERVICE \
  NS_IMETHOD GetApplicationAccessible(nsIAccessible **_retval) override; \
  NS_IMETHOD GetAccessibleFor(nsINode *aNode, nsIAccessible **_retval) override; \
  NS_IMETHOD GetStringRole(uint32_t aRole, nsAString& _retval) override; \
  NS_IMETHOD GetStringStates(uint32_t aStates, uint32_t aExtraStates, nsISupports **_retval) override; \
  NS_IMETHOD GetStringEventType(uint32_t aEventType, nsAString& _retval) override; \
  NS_IMETHOD GetStringRelationType(uint32_t aRelationType, nsAString& _retval) override; \
  NS_IMETHOD GetAccessibleFromCache(nsINode *aNode, nsIAccessible **_retval) override; \
  NS_IMETHOD CreateAccessiblePivot(nsIAccessible *aRoot, nsIAccessiblePivot **_retval) override; \
  NS_IMETHOD SetLogging(const nsACString& aModules) override; \
  NS_IMETHOD IsLogged(const nsAString& aModule, bool *_retval) override; \
  NS_IMETHOD GetConsumers(nsAString& _retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIACCESSIBILITYSERVICE \
  nsresult GetApplicationAccessible(nsIAccessible **_retval); \
  nsresult GetAccessibleFor(nsINode *aNode, nsIAccessible **_retval); \
  nsresult GetStringRole(uint32_t aRole, nsAString& _retval); \
  nsresult GetStringStates(uint32_t aStates, uint32_t aExtraStates, nsISupports **_retval); \
  nsresult GetStringEventType(uint32_t aEventType, nsAString& _retval); \
  nsresult GetStringRelationType(uint32_t aRelationType, nsAString& _retval); \
  nsresult GetAccessibleFromCache(nsINode *aNode, nsIAccessible **_retval); \
  nsresult CreateAccessiblePivot(nsIAccessible *aRoot, nsIAccessiblePivot **_retval); \
  nsresult SetLogging(const nsACString& aModules); \
  nsresult IsLogged(const nsAString& aModule, bool *_retval); \
  nsresult GetConsumers(nsAString& _retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIACCESSIBILITYSERVICE(_to) \
  NS_IMETHOD GetApplicationAccessible(nsIAccessible **_retval) override { return _to GetApplicationAccessible(_retval); } \
  NS_IMETHOD GetAccessibleFor(nsINode *aNode, nsIAccessible **_retval) override { return _to GetAccessibleFor(aNode, _retval); } \
  NS_IMETHOD GetStringRole(uint32_t aRole, nsAString& _retval) override { return _to GetStringRole(aRole, _retval); } \
  NS_IMETHOD GetStringStates(uint32_t aStates, uint32_t aExtraStates, nsISupports **_retval) override { return _to GetStringStates(aStates, aExtraStates, _retval); } \
  NS_IMETHOD GetStringEventType(uint32_t aEventType, nsAString& _retval) override { return _to GetStringEventType(aEventType, _retval); } \
  NS_IMETHOD GetStringRelationType(uint32_t aRelationType, nsAString& _retval) override { return _to GetStringRelationType(aRelationType, _retval); } \
  NS_IMETHOD GetAccessibleFromCache(nsINode *aNode, nsIAccessible **_retval) override { return _to GetAccessibleFromCache(aNode, _retval); } \
  NS_IMETHOD CreateAccessiblePivot(nsIAccessible *aRoot, nsIAccessiblePivot **_retval) override { return _to CreateAccessiblePivot(aRoot, _retval); } \
  NS_IMETHOD SetLogging(const nsACString& aModules) override { return _to SetLogging(aModules); } \
  NS_IMETHOD IsLogged(const nsAString& aModule, bool *_retval) override { return _to IsLogged(aModule, _retval); } \
  NS_IMETHOD GetConsumers(nsAString& _retval) override { return _to GetConsumers(_retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIACCESSIBILITYSERVICE(_to) \
  NS_IMETHOD GetApplicationAccessible(nsIAccessible **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetApplicationAccessible(_retval); } \
  NS_IMETHOD GetAccessibleFor(nsINode *aNode, nsIAccessible **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAccessibleFor(aNode, _retval); } \
  NS_IMETHOD GetStringRole(uint32_t aRole, nsAString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetStringRole(aRole, _retval); } \
  NS_IMETHOD GetStringStates(uint32_t aStates, uint32_t aExtraStates, nsISupports **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetStringStates(aStates, aExtraStates, _retval); } \
  NS_IMETHOD GetStringEventType(uint32_t aEventType, nsAString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetStringEventType(aEventType, _retval); } \
  NS_IMETHOD GetStringRelationType(uint32_t aRelationType, nsAString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetStringRelationType(aRelationType, _retval); } \
  NS_IMETHOD GetAccessibleFromCache(nsINode *aNode, nsIAccessible **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAccessibleFromCache(aNode, _retval); } \
  NS_IMETHOD CreateAccessiblePivot(nsIAccessible *aRoot, nsIAccessiblePivot **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CreateAccessiblePivot(aRoot, _retval); } \
  NS_IMETHOD SetLogging(const nsACString& aModules) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetLogging(aModules); } \
  NS_IMETHOD IsLogged(const nsAString& aModule, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->IsLogged(aModule, _retval); } \
  NS_IMETHOD GetConsumers(nsAString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetConsumers(_retval); } 


#endif /* __gen_nsIAccessibilityService_h__ */
