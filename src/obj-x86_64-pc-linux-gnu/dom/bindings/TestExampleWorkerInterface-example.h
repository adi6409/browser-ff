/* -*- Mode: C++; tab-width: 2; indent-tabs-mode: nil; c-basic-offset: 2 -*- */
/* vim:set ts=2 sw=2 sts=2 et cindent: */
/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#ifndef mozilla_dom_TestExampleWorkerInterface_h
#define mozilla_dom_TestExampleWorkerInterface_h

#include "js/TypeDecls.h"
#include "mozilla/Attributes.h"
#include "mozilla/ErrorResult.h"
#include "mozilla/dom/BindingDeclarations.h"
#include "nsCycleCollectionParticipant.h"
#include "nsWrapperCache.h"

namespace mozilla {
namespace dom {

class TestExampleWorkerInterface final : public nsISupports /* or NonRefcountedDOMObject if this is a non-refcounted object */,
                                         public nsWrapperCache /* Change wrapperCache in the binding configuration if you don't want this */
{
public:
  NS_DECL_CYCLE_COLLECTING_ISUPPORTS
  NS_DECL_CYCLE_COLLECTION_SCRIPT_HOLDER_CLASS(TestExampleWorkerInterface)

public:
  TestExampleWorkerInterface();

protected:
  ~TestExampleWorkerInterface();

public:
  // This should return something that eventually allows finding a
  // path to the global this object is associated with.  Most simply,
  // returning an actual global works.
  nsIGlobalObject* GetParentObject() const;

  JSObject* WrapObject(JSContext* aCx, JS::Handle<JSObject*> aGivenProto) override;

  void NeedsSubjectPrincipalMethod(Maybe<nsIPrincipal*> aSubjectPrincipal);

  bool NeedsSubjectPrincipalAttr(Maybe<nsIPrincipal*> aSubjectPrincipal) const;

  void SetNeedsSubjectPrincipalAttr(bool arg, Maybe<nsIPrincipal*> aSubjectPrincipal);

  void NeedsCallerTypeMethod(CallerType aCallerType);

  bool NeedsCallerTypeAttr(CallerType aCallerType) const;

  void SetNeedsCallerTypeAttr(bool arg, CallerType aCallerType);

  void NeedsNonSystemSubjectPrincipalMethod(Maybe<nsIPrincipal*> aSubjectPrincipal);

  bool NeedsNonSystemSubjectPrincipalAttr(Maybe<nsIPrincipal*> aSubjectPrincipal) const;

  void SetNeedsNonSystemSubjectPrincipalAttr(bool arg, Maybe<nsIPrincipal*> aSubjectPrincipal);
};

} // namespace dom
} // namespace mozilla

#endif // mozilla_dom_TestExampleWorkerInterface_h
