/* -*- Mode: C++; tab-width: 2; indent-tabs-mode: nil; c-basic-offset: 2 -*- */
/* vim:set ts=2 sw=2 sts=2 et cindent: */
/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#ifndef mozilla_dom_TestExampleProxyInterface_h
#define mozilla_dom_TestExampleProxyInterface_h

#include "js/TypeDecls.h"
#include "mozilla/Attributes.h"
#include "mozilla/ErrorResult.h"
#include "mozilla/dom/BindingDeclarations.h"
#include "nsCycleCollectionParticipant.h"
#include "nsWrapperCache.h"

namespace mozilla {
namespace dom {

class TestExampleProxyInterface final : public nsISupports /* or NonRefcountedDOMObject if this is a non-refcounted object */,
                                        public nsWrapperCache /* Change wrapperCache in the binding configuration if you don't want this */
{
public:
  NS_DECL_CYCLE_COLLECTING_ISUPPORTS
  NS_DECL_CYCLE_COLLECTION_SCRIPT_HOLDER_CLASS(TestExampleProxyInterface)

public:
  TestExampleProxyInterface();

protected:
  ~TestExampleProxyInterface();

public:
  // This should return something that eventually allows finding a
  // path to the global this object is associated with.  Most simply,
  // returning an actual global works.
  nsIGlobalObject* GetParentObject() const;

  JSObject* WrapObject(JSContext* aCx, JS::Handle<JSObject*> aGivenProto) override;

  int32_t LongIndexedGetter(uint32_t ix);

  void LongIndexedSetter(uint32_t y, int32_t z);

  uint32_t Length() const;

  void MyStringifier(nsString& aRetVal);

  int16_t ShortNameGetter(const nsAString& nom);

  void ShortNamedSetter(const nsAString& me, int16_t value);

  int32_t IndexedGetter(uint32_t ix, bool &found);

  void IndexedSetter(uint32_t y, int32_t z);

  void NamedDeleter(const nsAString& nomnom, bool &found);

  int16_t NamedGetter(const nsAString& nom, bool &found);

  void NamedSetter(const nsAString& me, int16_t value);

  void GetSupportedNames(nsTArray<nsString>& aRetVal);
};

} // namespace dom
} // namespace mozilla

#endif // mozilla_dom_TestExampleProxyInterface_h
