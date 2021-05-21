/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIPermissionManager.idl
 */

#ifndef __gen_nsIPermissionManager_h__
#define __gen_nsIPermissionManager_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#ifndef __gen_nsTArray_h__
#include "nsTArray.h"
#endif

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIPrincipal; /* forward declaration */

class nsIPermission; /* forward declaration */


/* starting interface:    nsIPermissionManager */
#define NS_IPERMISSIONMANAGER_IID_STR "4dcb3851-eba2-4e42-b236-82d2596fca22"

#define NS_IPERMISSIONMANAGER_IID \
  {0x4dcb3851, 0xeba2, 0x4e42, \
    { 0xb2, 0x36, 0x82, 0xd2, 0x59, 0x6f, 0xca, 0x22 }}

class NS_NO_VTABLE nsIPermissionManager : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IPERMISSIONMANAGER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIPermissionManager;

  enum {
    UNKNOWN_ACTION = 0U,
    ALLOW_ACTION = 1U,
    DENY_ACTION = 2U,
    PROMPT_ACTION = 3U,
    EXPIRE_NEVER = 0U,
    EXPIRE_SESSION = 1U,
    EXPIRE_TIME = 2U,
    EXPIRE_POLICY = 3U
  };

  /* Array<nsIPermission> getAllForPrincipal (in nsIPrincipal principal); */
  NS_IMETHOD GetAllForPrincipal(nsIPrincipal *principal, nsTArray<RefPtr<nsIPermission>>& _retval) = 0;

  /* Array<nsIPermission> getAllWithTypePrefix (in ACString prefix); */
  NS_IMETHOD GetAllWithTypePrefix(const nsACString& prefix, nsTArray<RefPtr<nsIPermission>>& _retval) = 0;

  /* Array<nsIPermission> getAllByTypeSince (in ACString type, in int64_t since); */
  NS_IMETHOD GetAllByTypeSince(const nsACString& type, int64_t since, nsTArray<RefPtr<nsIPermission>>& _retval) = 0;

  /* void addFromPrincipal (in nsIPrincipal principal, in ACString type, in uint32_t permission, [optional] in uint32_t expireType, [optional] in int64_t expireTime); */
  NS_IMETHOD AddFromPrincipal(nsIPrincipal *principal, const nsACString& type, uint32_t permission, uint32_t expireType, int64_t expireTime) = 0;

  /* void removeFromPrincipal (in nsIPrincipal principal, in ACString type); */
  NS_IMETHOD RemoveFromPrincipal(nsIPrincipal *principal, const nsACString& type) = 0;

  /* void removePermission (in nsIPermission perm); */
  NS_IMETHOD RemovePermission(nsIPermission *perm) = 0;

  /* void removeAll (); */
  NS_IMETHOD RemoveAll(void) = 0;

  /* void removeAllSince (in int64_t since); */
  NS_IMETHOD RemoveAllSince(int64_t since) = 0;

  /* void removeByType (in ACString type); */
  NS_IMETHOD RemoveByType(const nsACString& type) = 0;

  /* void removeByTypeSince (in ACString type, in int64_t since); */
  NS_IMETHOD RemoveByTypeSince(const nsACString& type, int64_t since) = 0;

  /* uint32_t testPermissionFromPrincipal (in nsIPrincipal principal, in ACString type); */
  NS_IMETHOD TestPermissionFromPrincipal(nsIPrincipal *principal, const nsACString& type, uint32_t *_retval) = 0;

  /* uint32_t testExactPermissionFromPrincipal (in nsIPrincipal principal, in ACString type); */
  NS_IMETHOD TestExactPermissionFromPrincipal(nsIPrincipal *principal, const nsACString& type, uint32_t *_retval) = 0;

  /* uint32_t testExactPermanentPermission (in nsIPrincipal principal, in ACString type); */
  NS_IMETHOD TestExactPermanentPermission(nsIPrincipal *principal, const nsACString& type, uint32_t *_retval) = 0;

  /* nsIPermission getPermissionObject (in nsIPrincipal principal, in ACString type, in boolean exactHost); */
  NS_IMETHOD GetPermissionObject(nsIPrincipal *principal, const nsACString& type, bool exactHost, nsIPermission **_retval) = 0;

  /* readonly attribute Array<nsIPermission> all; */
  NS_IMETHOD GetAll(nsTArray<RefPtr<nsIPermission>>& aAll) = 0;

  /* void removePermissionsWithAttributes (in AString patternAsJSON); */
  NS_IMETHOD RemovePermissionsWithAttributes(const nsAString& patternAsJSON) = 0;

  /* void broadcastPermissionsForPrincipalToAllContentProcesses (in nsIPrincipal aPrincipal); */
  NS_IMETHOD BroadcastPermissionsForPrincipalToAllContentProcesses(nsIPrincipal *aPrincipal) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIPermissionManager, NS_IPERMISSIONMANAGER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIPERMISSIONMANAGER \
  NS_IMETHOD GetAllForPrincipal(nsIPrincipal *principal, nsTArray<RefPtr<nsIPermission>>& _retval) override; \
  NS_IMETHOD GetAllWithTypePrefix(const nsACString& prefix, nsTArray<RefPtr<nsIPermission>>& _retval) override; \
  NS_IMETHOD GetAllByTypeSince(const nsACString& type, int64_t since, nsTArray<RefPtr<nsIPermission>>& _retval) override; \
  NS_IMETHOD AddFromPrincipal(nsIPrincipal *principal, const nsACString& type, uint32_t permission, uint32_t expireType, int64_t expireTime) override; \
  NS_IMETHOD RemoveFromPrincipal(nsIPrincipal *principal, const nsACString& type) override; \
  NS_IMETHOD RemovePermission(nsIPermission *perm) override; \
  NS_IMETHOD RemoveAll(void) override; \
  NS_IMETHOD RemoveAllSince(int64_t since) override; \
  NS_IMETHOD RemoveByType(const nsACString& type) override; \
  NS_IMETHOD RemoveByTypeSince(const nsACString& type, int64_t since) override; \
  NS_IMETHOD TestPermissionFromPrincipal(nsIPrincipal *principal, const nsACString& type, uint32_t *_retval) override; \
  NS_IMETHOD TestExactPermissionFromPrincipal(nsIPrincipal *principal, const nsACString& type, uint32_t *_retval) override; \
  NS_IMETHOD TestExactPermanentPermission(nsIPrincipal *principal, const nsACString& type, uint32_t *_retval) override; \
  NS_IMETHOD GetPermissionObject(nsIPrincipal *principal, const nsACString& type, bool exactHost, nsIPermission **_retval) override; \
  NS_IMETHOD GetAll(nsTArray<RefPtr<nsIPermission>>& aAll) override; \
  NS_IMETHOD RemovePermissionsWithAttributes(const nsAString& patternAsJSON) override; \
  NS_IMETHOD BroadcastPermissionsForPrincipalToAllContentProcesses(nsIPrincipal *aPrincipal) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIPERMISSIONMANAGER \
  nsresult GetAllForPrincipal(nsIPrincipal *principal, nsTArray<RefPtr<nsIPermission>>& _retval); \
  nsresult GetAllWithTypePrefix(const nsACString& prefix, nsTArray<RefPtr<nsIPermission>>& _retval); \
  nsresult GetAllByTypeSince(const nsACString& type, int64_t since, nsTArray<RefPtr<nsIPermission>>& _retval); \
  nsresult AddFromPrincipal(nsIPrincipal *principal, const nsACString& type, uint32_t permission, uint32_t expireType, int64_t expireTime); \
  nsresult RemoveFromPrincipal(nsIPrincipal *principal, const nsACString& type); \
  nsresult RemovePermission(nsIPermission *perm); \
  nsresult RemoveAll(void); \
  nsresult RemoveAllSince(int64_t since); \
  nsresult RemoveByType(const nsACString& type); \
  nsresult RemoveByTypeSince(const nsACString& type, int64_t since); \
  nsresult TestPermissionFromPrincipal(nsIPrincipal *principal, const nsACString& type, uint32_t *_retval); \
  nsresult TestExactPermissionFromPrincipal(nsIPrincipal *principal, const nsACString& type, uint32_t *_retval); \
  nsresult TestExactPermanentPermission(nsIPrincipal *principal, const nsACString& type, uint32_t *_retval); \
  nsresult GetPermissionObject(nsIPrincipal *principal, const nsACString& type, bool exactHost, nsIPermission **_retval); \
  nsresult GetAll(nsTArray<RefPtr<nsIPermission>>& aAll); \
  nsresult RemovePermissionsWithAttributes(const nsAString& patternAsJSON); \
  nsresult BroadcastPermissionsForPrincipalToAllContentProcesses(nsIPrincipal *aPrincipal); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIPERMISSIONMANAGER(_to) \
  NS_IMETHOD GetAllForPrincipal(nsIPrincipal *principal, nsTArray<RefPtr<nsIPermission>>& _retval) override { return _to GetAllForPrincipal(principal, _retval); } \
  NS_IMETHOD GetAllWithTypePrefix(const nsACString& prefix, nsTArray<RefPtr<nsIPermission>>& _retval) override { return _to GetAllWithTypePrefix(prefix, _retval); } \
  NS_IMETHOD GetAllByTypeSince(const nsACString& type, int64_t since, nsTArray<RefPtr<nsIPermission>>& _retval) override { return _to GetAllByTypeSince(type, since, _retval); } \
  NS_IMETHOD AddFromPrincipal(nsIPrincipal *principal, const nsACString& type, uint32_t permission, uint32_t expireType, int64_t expireTime) override { return _to AddFromPrincipal(principal, type, permission, expireType, expireTime); } \
  NS_IMETHOD RemoveFromPrincipal(nsIPrincipal *principal, const nsACString& type) override { return _to RemoveFromPrincipal(principal, type); } \
  NS_IMETHOD RemovePermission(nsIPermission *perm) override { return _to RemovePermission(perm); } \
  NS_IMETHOD RemoveAll(void) override { return _to RemoveAll(); } \
  NS_IMETHOD RemoveAllSince(int64_t since) override { return _to RemoveAllSince(since); } \
  NS_IMETHOD RemoveByType(const nsACString& type) override { return _to RemoveByType(type); } \
  NS_IMETHOD RemoveByTypeSince(const nsACString& type, int64_t since) override { return _to RemoveByTypeSince(type, since); } \
  NS_IMETHOD TestPermissionFromPrincipal(nsIPrincipal *principal, const nsACString& type, uint32_t *_retval) override { return _to TestPermissionFromPrincipal(principal, type, _retval); } \
  NS_IMETHOD TestExactPermissionFromPrincipal(nsIPrincipal *principal, const nsACString& type, uint32_t *_retval) override { return _to TestExactPermissionFromPrincipal(principal, type, _retval); } \
  NS_IMETHOD TestExactPermanentPermission(nsIPrincipal *principal, const nsACString& type, uint32_t *_retval) override { return _to TestExactPermanentPermission(principal, type, _retval); } \
  NS_IMETHOD GetPermissionObject(nsIPrincipal *principal, const nsACString& type, bool exactHost, nsIPermission **_retval) override { return _to GetPermissionObject(principal, type, exactHost, _retval); } \
  NS_IMETHOD GetAll(nsTArray<RefPtr<nsIPermission>>& aAll) override { return _to GetAll(aAll); } \
  NS_IMETHOD RemovePermissionsWithAttributes(const nsAString& patternAsJSON) override { return _to RemovePermissionsWithAttributes(patternAsJSON); } \
  NS_IMETHOD BroadcastPermissionsForPrincipalToAllContentProcesses(nsIPrincipal *aPrincipal) override { return _to BroadcastPermissionsForPrincipalToAllContentProcesses(aPrincipal); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIPERMISSIONMANAGER(_to) \
  NS_IMETHOD GetAllForPrincipal(nsIPrincipal *principal, nsTArray<RefPtr<nsIPermission>>& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAllForPrincipal(principal, _retval); } \
  NS_IMETHOD GetAllWithTypePrefix(const nsACString& prefix, nsTArray<RefPtr<nsIPermission>>& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAllWithTypePrefix(prefix, _retval); } \
  NS_IMETHOD GetAllByTypeSince(const nsACString& type, int64_t since, nsTArray<RefPtr<nsIPermission>>& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAllByTypeSince(type, since, _retval); } \
  NS_IMETHOD AddFromPrincipal(nsIPrincipal *principal, const nsACString& type, uint32_t permission, uint32_t expireType, int64_t expireTime) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AddFromPrincipal(principal, type, permission, expireType, expireTime); } \
  NS_IMETHOD RemoveFromPrincipal(nsIPrincipal *principal, const nsACString& type) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RemoveFromPrincipal(principal, type); } \
  NS_IMETHOD RemovePermission(nsIPermission *perm) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RemovePermission(perm); } \
  NS_IMETHOD RemoveAll(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RemoveAll(); } \
  NS_IMETHOD RemoveAllSince(int64_t since) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RemoveAllSince(since); } \
  NS_IMETHOD RemoveByType(const nsACString& type) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RemoveByType(type); } \
  NS_IMETHOD RemoveByTypeSince(const nsACString& type, int64_t since) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RemoveByTypeSince(type, since); } \
  NS_IMETHOD TestPermissionFromPrincipal(nsIPrincipal *principal, const nsACString& type, uint32_t *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->TestPermissionFromPrincipal(principal, type, _retval); } \
  NS_IMETHOD TestExactPermissionFromPrincipal(nsIPrincipal *principal, const nsACString& type, uint32_t *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->TestExactPermissionFromPrincipal(principal, type, _retval); } \
  NS_IMETHOD TestExactPermanentPermission(nsIPrincipal *principal, const nsACString& type, uint32_t *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->TestExactPermanentPermission(principal, type, _retval); } \
  NS_IMETHOD GetPermissionObject(nsIPrincipal *principal, const nsACString& type, bool exactHost, nsIPermission **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPermissionObject(principal, type, exactHost, _retval); } \
  NS_IMETHOD GetAll(nsTArray<RefPtr<nsIPermission>>& aAll) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAll(aAll); } \
  NS_IMETHOD RemovePermissionsWithAttributes(const nsAString& patternAsJSON) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RemovePermissionsWithAttributes(patternAsJSON); } \
  NS_IMETHOD BroadcastPermissionsForPrincipalToAllContentProcesses(nsIPrincipal *aPrincipal) override { return !_to ? NS_ERROR_NULL_POINTER : _to->BroadcastPermissionsForPrincipalToAllContentProcesses(aPrincipal); } 

#define NS_PERMISSIONMANAGER_CONTRACTID "@mozilla.org/permissionmanager;1"
#define PERM_CHANGE_NOTIFICATION "perm-changed"

#endif /* __gen_nsIPermissionManager_h__ */
