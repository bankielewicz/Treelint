# ADR-EXAMPLE-003: Zustand for React State Management

**Date**: 2025-10-25
**Status**: Accepted
**Deciders**: Frontend Lead, Senior Frontend Developers (2), UX Engineer
**Project**: E-Commerce Customer Portal (React SPA)

---

## Context

### Project Background
Building a customer-facing e-commerce portal with:
- **User Base**: 50,000+ registered customers
- **Page Types**: Product catalog, shopping cart, checkout, order history, user profile
- **State Complexity**: Medium (shopping cart, user session, filters, UI preferences)
- **Performance Target**: <2s page load, <100ms state updates
- **Team Size**: 4 frontend developers (2 senior, 1 mid-level, 1 junior)
- **Technology Stack**: React 18, TypeScript 5, Vite, Tailwind CSS

### Problem Statement
We need a state management solution that balances:
1. **Developer Productivity**: Easy to learn, minimal boilerplate
2. **Performance**: Fast updates, no unnecessary re-renders
3. **TypeScript Support**: Full type safety for state and actions
4. **Bundle Size**: Keep client bundle <300KB (current: 180KB)
5. **Debugging**: Easy to track state changes and side effects

### Current Pain Points
Currently using **React Context + useReducer** for global state:
- ❌ **Boilerplate overload**: 100+ lines per context (provider, reducer, actions, types)
- ❌ **Re-render issues**: All consumers re-render on any state change
- ❌ **Type safety gaps**: Action types not fully type-safe
- ❌ **Testing complexity**: Difficult to test context providers in isolation
- ❌ **DevTools limitations**: No time-travel debugging

### Requirements

**Functional Requirements**:
- **Shopping Cart State**: Add/remove items, update quantities, persist to localStorage
- **User Session State**: Auth status, user profile, preferences
- **UI State**: Modal visibility, loading states, notifications
- **Filter State**: Product filters (category, price range, rating)
- **Async State**: API call status, loading indicators, error handling

**Non-Functional Requirements**:
- **Performance**: <100ms state update latency
- **Bundle Size Impact**: <20KB gzipped (add to current 180KB bundle)
- **TypeScript**: 100% type-safe state and actions
- **DevTools**: Time-travel debugging, state inspection
- **Learning Curve**: New developers productive within 3 days

### Constraints
- Must work with React 18 (concurrent features)
- Must support server-side rendering (future requirement)
- Must integrate with existing React Query for API data
- Must persist cart state to localStorage automatically
- No breaking changes to existing codebase (gradual migration)

---

## Decision

**We will use Zustand as the primary state management solution.**

Specifically:
- **Zustand 4.x** for global state management
- **React Query 4.x** for server state (already in use, not replaced)
- **Local useState/useReducer** for component-specific state
- **Persist middleware** for localStorage synchronization

---

## Rationale

### Technical Rationale

#### 1. Simplicity and Developer Experience
Zustand requires **minimal boilerplate** compared to alternatives:

**Zustand Example** (Shopping Cart Store):
```typescript
// stores/cartStore.ts (45 lines total)
import { create } from 'zustand';
import { persist } from 'zustand/middleware';

interface CartItem {
  productId: number;
  name: string;
  price: number;
  quantity: number;
}

interface CartState {
  items: CartItem[];
  addItem: (item: CartItem) => void;
  removeItem: (productId: number) => void;
  updateQuantity: (productId: number, quantity: number) => void;
  clearCart: () => void;
  total: () => number;
}

export const useCartStore = create<CartState>()(
  persist(
    (set, get) => ({
      items: [],

      addItem: (item) =>
        set((state) => {
          const existing = state.items.find((i) => i.productId === item.productId);
          if (existing) {
            return {
              items: state.items.map((i) =>
                i.productId === item.productId
                  ? { ...i, quantity: i.quantity + item.quantity }
                  : i
              ),
            };
          }
          return { items: [...state.items, item] };
        }),

      removeItem: (productId) =>
        set((state) => ({
          items: state.items.filter((i) => i.productId !== productId),
        })),

      updateQuantity: (productId, quantity) =>
        set((state) => ({
          items: state.items.map((i) =>
            i.productId === productId ? { ...i, quantity } : i
          ),
        })),

      clearCart: () => set({ items: [] }),

      total: () => {
        const { items } = get();
        return items.reduce((sum, item) => sum + item.price * item.quantity, 0);
      },
    }),
    {
      name: 'cart-storage', // localStorage key
    }
  )
);
```

**Usage in Component**:
```tsx
// components/ShoppingCart.tsx
import { useCartStore } from '@/stores/cartStore';

export const ShoppingCart = () => {
  const items = useCartStore((state) => state.items);
  const removeItem = useCartStore((state) => state.removeItem);
  const total = useCartStore((state) => state.total());

  return (
    <div>
      {items.map((item) => (
        <div key={item.productId}>
          {item.name} - ${item.price} x {item.quantity}
          <button onClick={() => removeItem(item.productId)}>Remove</button>
        </div>
      ))}
      <div>Total: ${total}</div>
    </div>
  );
};
```

**Key Advantages**:
- ✅ No provider boilerplate (no `<CartProvider>` wrapping)
- ✅ Direct store access via hook (`useCartStore`)
- ✅ Selector optimization built-in (only re-render when selected state changes)
- ✅ Middleware support (persist to localStorage with 2 lines)
- ✅ Fully type-safe (TypeScript infers all types)

#### 2. Performance Optimization
Zustand prevents unnecessary re-renders through **fine-grained selectors**:

**Problem with Context API**:
```tsx
// ❌ BAD: Re-renders on any cart state change
const { items, addItem, removeItem, total } = useCart();
// Even if component only uses `total`, it re-renders when `items` change
```

**Zustand Solution**:
```tsx
// ✅ GOOD: Only re-renders when `total` changes
const total = useCartStore((state) => state.total());
// Component ignores `items` changes unless `total` changes
```

**Benchmark** (React 18, 1000 state updates):
```
Redux Toolkit:   47ms average (middleware overhead)
Zustand:         12ms average (direct state mutation)
React Context:   134ms average (provider cascade)
Jotai:           23ms average (atom diffing)

Re-render count (1000 updates to cart items):
Redux Toolkit:   1000 re-renders (all subscribers)
Zustand:         312 re-renders (optimized selectors)
React Context:   1000 re-renders (all consumers)
Jotai:           287 re-renders (optimized subscriptions)
```

**Memory Usage**:
- Zustand: ~3KB per store instance
- Redux Toolkit: ~12KB (store + middleware + devtools)
- Context: ~1KB per context (but multiple contexts = management overhead)

#### 3. TypeScript Integration
Zustand provides **100% type safety** with minimal effort:

```typescript
// Full type inference without manual typing
const items = useCartStore((state) => state.items); // CartItem[]
const addItem = useCartStore((state) => state.addItem); // (item: CartItem) => void

// TypeScript catches errors
addItem({ productId: 1, name: 'Widget' }); // ❌ Error: Missing `price` and `quantity`

// Autocomplete for actions and state
useCartStore((state) => state. /* autocomplete shows: items, addItem, removeItem, etc. */
```

**Compared to Redux**:
Redux Toolkit requires more type definitions:
```typescript
// Redux: Define action types, payload types, state types separately
interface AddItemAction {
  type: 'cart/addItem';
  payload: CartItem;
}

// Zustand: Types inferred from implementation
// No separate action type definitions needed
```

#### 4. Bundle Size Impact
**Package Sizes (gzipped)**:
- **Zustand**: 3.2KB (core)
- **Persist middleware**: 1.1KB
- **DevTools middleware**: 0.8KB
- **Total**: ~5KB gzipped

**Comparison**:
- Redux Toolkit: 14.2KB (core + thunk + devtools)
- Jotai: 3.8KB (core + utils)
- React Context: 0KB (built-in, but boilerplate adds project code)
- MobX: 16.7KB (core + react bindings)

**Impact on Bundle**:
- Current bundle: 180KB gzipped
- After Zustand: 185KB gzipped (+2.7% increase)
- Well under 300KB target ✅

#### 5. Learning Curve
**Time to Productivity**:
- **Junior Developer**: 1-2 days (read docs, implement simple store)
- **Mid-Level Developer**: 4 hours (familiar with hooks, understands selectors)
- **Senior Developer**: 1 hour (grasps concept immediately)

**Compared to Alternatives**:
- **Redux Toolkit**: 1-2 weeks (actions, reducers, slices, middleware, thunks)
- **MobX**: 3-5 days (observables, reactions, decorators, class components)
- **Jotai**: 2-3 days (atom concept, providers, derived atoms)
- **React Context**: 0 days (already know it, but boilerplate is painful)

**Training Materials Needed**:
- ✅ 1-hour internal workshop: "Zustand Basics"
- ✅ Code examples in team wiki (cart, auth, UI state)
- ✅ Migration guide: Context → Zustand

#### 6. Middleware Ecosystem
Zustand's middleware system solves common problems:

**Persist Middleware** (localStorage sync):
```typescript
import { persist } from 'zustand/middleware';

export const useCartStore = create<CartState>()(
  persist(
    (set, get) => ({ /* state */ }),
    { name: 'cart-storage' } // Automatically saves to localStorage
  )
);
```

**DevTools Middleware** (Redux DevTools integration):
```typescript
import { devtools } from 'zustand/middleware';

export const useCartStore = create<CartState>()(
  devtools(
    (set, get) => ({ /* state */ }),
    { name: 'CartStore' } // Shows in Redux DevTools
  )
);
```

**Immer Middleware** (immutable updates):
```typescript
import { immer } from 'zustand/middleware/immer';

export const useCartStore = create<CartState>()(
  immer((set) => ({
    items: [],
    addItem: (item) =>
      set((state) => {
        state.items.push(item); // Direct mutation (Immer makes it immutable)
      }),
  }))
);
```

**Custom Middleware** (logging, analytics):
```typescript
const logMiddleware = (config) => (set, get, api) =>
  config(
    (...args) => {
      console.log('State before:', get());
      set(...args);
      console.log('State after:', get());
    },
    get,
    api
  );

export const useCartStore = create(logMiddleware((set) => ({ /* state */ })));
```

#### 7. Debugging Experience
**Redux DevTools Integration**:
- Time-travel debugging (replay actions)
- State inspection (view full state tree)
- Action history (see all state changes)

**Console Logging**:
```typescript
// Get current state anytime (outside React components)
console.log(useCartStore.getState());

// Subscribe to changes
const unsub = useCartStore.subscribe((state) => {
  console.log('Cart updated:', state.items);
});
```

#### 8. Testing Benefits
Zustand stores are **easy to test** (no providers needed):

```typescript
// cartStore.test.ts
import { renderHook, act } from '@testing-library/react';
import { useCartStore } from './cartStore';

describe('CartStore', () => {
  beforeEach(() => {
    // Reset store before each test
    useCartStore.setState({ items: [] });
  });

  it('adds item to cart', () => {
    const { result } = renderHook(() => useCartStore());

    act(() => {
      result.current.addItem({
        productId: 1,
        name: 'Widget',
        price: 19.99,
        quantity: 2,
      });
    });

    expect(result.current.items).toHaveLength(1);
    expect(result.current.total()).toBe(39.98);
  });

  it('increments quantity if item already exists', () => {
    const { result } = renderHook(() => useCartStore());

    const item = { productId: 1, name: 'Widget', price: 19.99, quantity: 1 };

    act(() => {
      result.current.addItem(item);
      result.current.addItem(item);
    });

    expect(result.current.items).toHaveLength(1);
    expect(result.current.items[0].quantity).toBe(2);
  });
});
```

**No Provider Wrappers Needed**:
```tsx
// ❌ Redux: Requires store wrapper
const { result } = renderHook(() => useSelector(selectCart), {
  wrapper: ({ children }) => <Provider store={store}>{children}</Provider>,
});

// ✅ Zustand: Direct hook testing
const { result } = renderHook(() => useCartStore());
```

---

## Consequences

### Positive Consequences

#### 1. Developer Productivity Gains (Quantified)
- **80% less boilerplate** vs Redux Toolkit (45 lines vs 200+ lines for cart store)
- **90% less boilerplate** vs React Context (45 lines vs 400+ lines with provider, reducer, types)
- **Faster feature development**: New state feature takes 15-30 minutes (vs 1-2 hours with Redux)
- **Easier refactoring**: Moving state between stores is simple (copy-paste, no action wiring)

#### 2. Performance Improvements
- **50% fewer re-renders** compared to Context API (selector optimization)
- **4x faster state updates** compared to Context (12ms vs 134ms benchmark)
- **Smaller bundle size**: +5KB vs +14KB (Redux Toolkit)
- **Lower memory footprint**: ~3KB per store vs ~12KB (Redux)

#### 3. Improved Developer Experience
- **Instant gratification**: New developers productive in 1-2 days
- **Better debugging**: Redux DevTools integration out-of-the-box
- **Type safety**: Full TypeScript inference, no manual type definitions
- **Simplified testing**: No provider wrappers, easy to mock stores

#### 4. Future-Proofing
- **Server-side rendering ready**: Zustand supports SSR (Next.js compatible)
- **Concurrent React support**: Works with React 18 concurrent features
- **Composability**: Multiple stores can be composed (no global store lock-in)
- **Migration path**: Can coexist with existing Context API (gradual migration)

### Negative Consequences

#### 1. Less Prescriptive Architecture
**Impact**: Low
**Challenge**: Zustand doesn't enforce patterns (freedom = potential inconsistency)

**Example Risk**:
```typescript
// ❌ Bad: Direct state mutation (breaks immutability)
set((state) => {
  state.items[0].quantity = 5; // Mutates existing object
  return state;
});

// ✅ Good: Immutable update
set((state) => ({
  items: state.items.map((item, idx) =>
    idx === 0 ? { ...item, quantity: 5 } : item
  ),
}));
```

**Mitigation**:
- Use **Immer middleware** for simpler immutable updates:
  ```typescript
  import { immer } from 'zustand/middleware/immer';

  export const useCartStore = create(immer((set) => ({
    items: [],
    updateQuantity: (productId, quantity) =>
      set((state) => {
        const item = state.items.find((i) => i.productId === productId);
        if (item) item.quantity = quantity; // Direct mutation (Immer handles immutability)
      }),
  })));
  ```
- **Document patterns** in team wiki (store structure, naming conventions)
- **Code review checklist**: Verify immutable updates, no direct mutations

#### 2. No Built-in Async Middleware
**Impact**: Low (React Query handles most async needs)
**Challenge**: Zustand has no built-in thunk/saga equivalent for async actions

**Comparison**:
- **Redux**: Redux Toolkit includes `createAsyncThunk` for API calls
- **Zustand**: No built-in async middleware (use React Query or async actions)

**Our Solution** (Zustand + React Query):
```typescript
// Zustand: UI state only
export const useUIStore = create<UIState>((set) => ({
  isLoading: false,
  setLoading: (isLoading) => set({ isLoading }),
}));

// React Query: API data
export const useProducts = () => {
  const { data, isLoading, error } = useQuery({
    queryKey: ['products'],
    queryFn: fetchProducts,
  });

  return { products: data, isLoading, error };
};

// Component: Combines both
export const ProductList = () => {
  const { products, isLoading } = useProducts();
  const isUILoading = useUIStore((state) => state.isLoading);

  if (isLoading || isUILoading) return <Spinner />;

  return <div>{products.map((p) => <ProductCard key={p.id} {...p} />)}</div>;
};
```

**Why This Works**:
- React Query handles server state (API caching, refetching, optimistic updates)
- Zustand handles client state (UI, cart, preferences)
- Clear separation of concerns

**Alternative** (if React Query not available):
```typescript
// Zustand: Async actions
export const useProductStore = create<ProductState>((set) => ({
  products: [],
  isLoading: false,
  error: null,

  fetchProducts: async () => {
    set({ isLoading: true, error: null });
    try {
      const products = await api.getProducts();
      set({ products, isLoading: false });
    } catch (error) {
      set({ error: error.message, isLoading: false });
    }
  },
}));
```

#### 3. Community Size Smaller Than Redux
**Impact**: Low
**Stats** (GitHub, 2025-10):
- Redux: 60K stars, 15K dependent repos
- Zustand: 45K stars, 8K dependent repos
- Redux Toolkit: 10K stars (part of Redux ecosystem)

**Reality Check**:
- Zustand is **mature** (5+ years, stable API)
- Active maintenance (frequent releases, responsive maintainers)
- Large enough community for troubleshooting (Stack Overflow, Discord)
- Growing faster than Redux (momentum shifted post-2020)

**Mitigation**:
- Document internal patterns in team wiki (reduce reliance on external tutorials)
- Create internal training materials (Zustand + React Query combo)
- Senior developers mentor juniors (knowledge transfer)

#### 4. Global State Can Be Misused
**Impact**: Medium (architectural risk)
**Anti-Pattern**: Using Zustand for everything (including local component state)

**❌ Bad Example**:
```typescript
// Overuse: Modal state in global store (should be local)
export const useModalStore = create<ModalState>((set) => ({
  isOpen: false,
  openModal: () => set({ isOpen: true }),
  closeModal: () => set({ isOpen: false }),
}));

// Component unnecessarily coupled to global state
export const Modal = () => {
  const { isOpen, closeModal } = useModalStore();
  // ...
};
```

**✅ Good Example**:
```typescript
// Local state for component-specific UI
export const Modal = ({ onClose }) => {
  const [isOpen, setIsOpen] = useState(false);
  // ...
};

// Global state only for shared data
export const useCartStore = create<CartState>((set) => ({
  items: [], // Shared across pages
  addItem: (item) => set(/* ... */),
}));
```

**Mitigation**:
- **Establish guidelines**:
  - Use Zustand for: Cart, auth, user preferences, theme, cross-page state
  - Use local state for: Form inputs, modals, tooltips, component-specific UI
- **Code review**: Flag inappropriate global state usage
- **Training**: Teach "lift state only when necessary"

---

## Alternatives Considered

### Alternative 1: Redux Toolkit

**Description**: Modern Redux with simplified API, built-in middleware (thunk, devtools).

**Pros**:
- **Mature ecosystem**: 60K GitHub stars, extensive docs, large community
- **Prescriptive patterns**: Clear conventions (slices, actions, reducers)
- **Built-in async**: `createAsyncThunk` for API calls
- **Middleware ecosystem**: Redux Saga, Redux Observable, etc.
- **Team familiarity**: Some developers have Redux experience

**Cons**:
- **Boilerplate overhead**: 200+ lines for cart store (vs 45 lines Zustand)
- **Learning curve**: 1-2 weeks for new developers (actions, reducers, slices)
- **Performance**: Slower updates (47ms vs 12ms Zustand)
- **Bundle size**: +14KB (vs +5KB Zustand)
- **Testing complexity**: Requires store provider wrapper

**Example Comparison**:
```typescript
// Redux Toolkit: 200+ lines
// features/cart/cartSlice.ts
import { createSlice, PayloadAction } from '@reduxjs/toolkit';

interface CartItem { /* ... */ }
interface CartState { items: CartItem[] }

const initialState: CartState = { items: [] };

const cartSlice = createSlice({
  name: 'cart',
  initialState,
  reducers: {
    addItem: (state, action: PayloadAction<CartItem>) => {
      const existing = state.items.find(i => i.productId === action.payload.productId);
      if (existing) {
        existing.quantity += action.payload.quantity;
      } else {
        state.items.push(action.payload);
      }
    },
    removeItem: (state, action: PayloadAction<number>) => {
      state.items = state.items.filter(i => i.productId !== action.payload);
    },
    // ... more actions
  },
});

export const { addItem, removeItem } = cartSlice.actions;
export default cartSlice.reducer;

// store.ts
import { configureStore } from '@reduxjs/toolkit';
import cartReducer from './features/cart/cartSlice';

export const store = configureStore({
  reducer: {
    cart: cartReducer,
  },
});

export type RootState = ReturnType<typeof store.getState>;
export type AppDispatch = typeof store.dispatch;

// App.tsx
import { Provider } from 'react-redux';
import { store } from './store';

<Provider store={store}>
  <App />
</Provider>

// Component usage
import { useSelector, useDispatch } from 'react-redux';
import { addItem, removeItem } from './features/cart/cartSlice';

const items = useSelector((state: RootState) => state.cart.items);
const dispatch = useDispatch();

dispatch(addItem({ productId: 1, name: 'Widget', price: 19.99, quantity: 1 }));
```

**vs Zustand (45 lines total)**: Already shown in "Decision" section above.

**Why Rejected**:
- **Boilerplate cost**: 4-5x more code for same functionality
- **Learning curve**: 2-3 weeks for team proficiency (vs 1-2 days Zustand)
- **Performance**: 4x slower updates (47ms vs 12ms)
- **Bundle size**: +14KB vs +5KB (78% larger)
- **Team feedback**: Developers prefer simplicity (internal survey: 85% favor Zustand)

**Conclusion**: Redux Toolkit is excellent for **large, complex applications** with strict architectural needs. Our e-commerce portal has **medium complexity** and benefits more from Zustand's simplicity.

---

### Alternative 2: Jotai

**Description**: Atomic state management library inspired by Recoil, built for React.

**Pros**:
- **Atomic model**: Fine-grained state updates (atom-level subscriptions)
- **Small bundle**: 3.8KB gzipped (vs 5KB Zustand)
- **Composable**: Derived atoms for computed state
- **TypeScript-first**: Full type inference
- **DevTools**: Similar Redux DevTools integration

**Cons**:
- **Provider required**: Must wrap app in `<Provider>` (unlike Zustand)
- **Steeper learning curve**: Atom concept is unfamiliar (2-3 days vs 1-2 days Zustand)
- **More abstraction**: Atoms + derived atoms + atom families (complexity)
- **Less straightforward**: Splitting state into atoms requires upfront design

**Example** (Jotai cart store):
```typescript
// atoms/cartAtoms.ts
import { atom } from 'jotai';

interface CartItem { /* ... */ }

export const cartItemsAtom = atom<CartItem[]>([]);

export const cartTotalAtom = atom((get) => {
  const items = get(cartItemsAtom);
  return items.reduce((sum, item) => sum + item.price * item.quantity, 0);
});

export const addItemAtom = atom(
  null,
  (get, set, item: CartItem) => {
    const items = get(cartItemsAtom);
    const existing = items.find(i => i.productId === item.productId);
    if (existing) {
      set(cartItemsAtom, items.map(i =>
        i.productId === item.productId
          ? { ...i, quantity: i.quantity + item.quantity }
          : i
      ));
    } else {
      set(cartItemsAtom, [...items, item]);
    }
  }
);

// Component usage
import { useAtom, useAtomValue } from 'jotai';
import { cartItemsAtom, addItemAtom, cartTotalAtom } from './atoms/cartAtoms';

const items = useAtomValue(cartItemsAtom);
const [, addItem] = useAtom(addItemAtom); // Write-only atom
const total = useAtomValue(cartTotalAtom);

addItem({ productId: 1, name: 'Widget', price: 19.99, quantity: 1 });
```

**Why Rejected**:
- **Provider requirement**: Adds boilerplate (`<Provider>` wrapper)
- **Atom abstraction**: Requires upfront design (which state becomes an atom?)
- **Learning curve**: Atom concept is less intuitive than Zustand's store
- **Team feedback**: Developers prefer Zustand's directness (internal survey: 70% favor Zustand)

**Conclusion**: Jotai is excellent for **highly dynamic state** with many computed values. Our cart state is straightforward and doesn't require atom-level granularity.

---

### Alternative 3: MobX

**Description**: Reactive state management library using observables and automatic tracking.

**Pros**:
- **Automatic tracking**: No manual subscriptions (MobX tracks dependencies)
- **Class-based**: Familiar OOP patterns (classes, decorators)
- **Computed values**: Automatic memoization
- **Mature**: 10+ years, stable API

**Cons**:
- **Large bundle**: 16.7KB gzipped (3x larger than Zustand)
- **Steep learning curve**: Observables, reactions, decorators (3-5 days)
- **Class-based**: Doesn't fit React's functional paradigm
- **Magic behavior**: Automatic tracking can be surprising (debugging issues)
- **Decorator syntax**: Requires TypeScript configuration (experimental decorators)

**Example** (MobX cart store):
```typescript
// stores/CartStore.ts
import { makeAutoObservable } from 'mobx';

interface CartItem { /* ... */ }

class CartStore {
  items: CartItem[] = [];

  constructor() {
    makeAutoObservable(this); // Automatic observables
  }

  addItem(item: CartItem) {
    const existing = this.items.find(i => i.productId === item.productId);
    if (existing) {
      existing.quantity += item.quantity;
    } else {
      this.items.push(item);
    }
  }

  removeItem(productId: number) {
    this.items = this.items.filter(i => i.productId !== productId);
  }

  get total() {
    return this.items.reduce((sum, item) => sum + item.price * item.quantity, 0);
  }
}

export const cartStore = new CartStore();

// Component usage
import { observer } from 'mobx-react-lite';
import { cartStore } from './stores/CartStore';

export const ShoppingCart = observer(() => {
  return (
    <div>
      {cartStore.items.map(item => (
        <div key={item.productId}>{item.name}</div>
      ))}
      <div>Total: ${cartStore.total}</div>
    </div>
  );
});
```

**Why Rejected**:
- **Bundle size**: 16.7KB (3x larger than Zustand, exceeds our +20KB target)
- **Learning curve**: 3-5 days for team proficiency (observables are unfamiliar)
- **Class-based**: Team prefers functional React patterns (hooks, not classes)
- **Complexity**: Automatic tracking is "magic" (harder to debug)
- **Declining adoption**: MobX usage declining in favor of simpler solutions

**Conclusion**: MobX is powerful for **complex, enterprise-scale applications** with heavy OOP patterns. Our team prefers functional React patterns, and MobX's bundle size exceeds our target.

---

### Alternative 4: React Context + useReducer (Current Solution)

**Description**: Built-in React state management using Context API and hooks.

**Pros**:
- **No dependencies**: Built into React (0KB bundle impact)
- **Team familiarity**: Everyone knows Context and useReducer
- **Simple for small apps**: Works well for single-context scenarios

**Cons**:
- **Boilerplate explosion**: 400+ lines for cart context (provider, reducer, actions, types)
- **Re-render issues**: All consumers re-render on any state change
- **Performance**: 10x slower than Zustand (134ms vs 12ms)
- **Testing complexity**: Requires provider wrapper in tests
- **No DevTools**: No time-travel debugging

**Example** (Context cart implementation - abbreviated):
```typescript
// contexts/CartContext.tsx (400+ lines)
import React, { createContext, useContext, useReducer } from 'react';

interface CartItem { /* ... */ }

interface CartState {
  items: CartItem[];
}

type CartAction =
  | { type: 'ADD_ITEM'; payload: CartItem }
  | { type: 'REMOVE_ITEM'; payload: number }
  | { type: 'UPDATE_QUANTITY'; payload: { productId: number; quantity: number } }
  | { type: 'CLEAR_CART' };

const CartContext = createContext<{
  state: CartState;
  dispatch: React.Dispatch<CartAction>;
} | undefined>(undefined);

function cartReducer(state: CartState, action: CartAction): CartState {
  switch (action.type) {
    case 'ADD_ITEM': {
      const existing = state.items.find(i => i.productId === action.payload.productId);
      if (existing) {
        return {
          items: state.items.map(i =>
            i.productId === action.payload.productId
              ? { ...i, quantity: i.quantity + action.payload.quantity }
              : i
          ),
        };
      }
      return { items: [...state.items, action.payload] };
    }
    case 'REMOVE_ITEM':
      return {
        items: state.items.filter(i => i.productId !== action.payload),
      };
    // ... more cases
    default:
      return state;
  }
}

export const CartProvider: React.FC<{ children: React.ReactNode }> = ({ children }) => {
  const [state, dispatch] = useReducer(cartReducer, { items: [] });

  // Persist to localStorage
  useEffect(() => {
    localStorage.setItem('cart', JSON.stringify(state.items));
  }, [state.items]);

  return (
    <CartContext.Provider value={{ state, dispatch }}>
      {children}
    </CartContext.Provider>
  );
};

export const useCart = () => {
  const context = useContext(CartContext);
  if (!context) throw new Error('useCart must be used within CartProvider');
  return context;
};

// Helper hooks
export const useCartItems = () => useCart().state.items;
export const useCartTotal = () => {
  const items = useCartItems();
  return items.reduce((sum, item) => sum + item.price * item.quantity, 0);
};

// Action creators
export const useCartActions = () => {
  const { dispatch } = useCart();
  return {
    addItem: (item: CartItem) => dispatch({ type: 'ADD_ITEM', payload: item }),
    removeItem: (productId: number) => dispatch({ type: 'REMOVE_ITEM', payload: productId }),
    updateQuantity: (productId: number, quantity: number) =>
      dispatch({ type: 'UPDATE_QUANTITY', payload: { productId, quantity } }),
    clearCart: () => dispatch({ type: 'CLEAR_CART' }),
  };
};

// App.tsx
<CartProvider>
  <App />
</CartProvider>
```

**Why Rejected (Current Pain Points)**:
- **Boilerplate**: 400+ lines vs 45 lines (Zustand) = 9x more code
- **Re-render hell**: Every consumer re-renders on any cart change (performance issue)
- **Testing pain**: Every test requires `<CartProvider>` wrapper
- **No DevTools**: Can't inspect state history or time-travel debug
- **Type safety gaps**: Action types require manual type guards

**Conclusion**: Context API is fine for **simple, single-context scenarios**. Our app has grown to **multiple contexts** (cart, auth, UI, filters), and the boilerplate is unmaintainable.

---

## Implementation Plan

### Phase 1: Setup and Training (Week 1)

**1. Install Dependencies**:
```bash
npm install zustand@4.x --save
npm install @types/node --save-dev # For middleware types
```

**2. Configure Middleware**:
```typescript
// lib/store-middleware.ts
import { StateCreator, StoreMutatorIdentifier } from 'zustand';
import { devtools, persist } from 'zustand/middleware';

// Typed middleware helper
export const createStore = <T extends object>(
  name: string,
  initializer: StateCreator<T, [['zustand/devtools', never], ['zustand/persist', unknown]]>
) =>
  devtools(
    persist(initializer, {
      name: `${name}-storage`,
    }),
    { name }
  );
```

**3. Team Training**:
- Workshop: "Zustand Fundamentals" (1 hour)
- Hands-on: Implement sample store (user preferences)
- Code review: Review first Zustand PR together

### Phase 2: Migrate Cart Store (Week 2)

**4. Create Cart Store**:
```typescript
// stores/useCartStore.ts
import { create } from 'zustand';
import { createStore } from '@/lib/store-middleware';

export const useCartStore = create(
  createStore<CartState>('Cart', (set, get) => ({
    items: [],
    addItem: (item) => {
      // Implementation shown earlier
    },
    // ... other actions
  }))
);
```

**5. Replace Cart Context**:
- Update components to use `useCartStore` instead of `useCart`
- Remove `<CartProvider>` from App.tsx
- Delete `contexts/CartContext.tsx`
- Run integration tests (cart functionality)

**6. Performance Validation**:
- Benchmark: Measure state update latency (target: <100ms)
- Re-render count: Verify selector optimization works
- Bundle size: Confirm <20KB increase

### Phase 3: Migrate Remaining State (Week 3-4)

**7. Create Additional Stores**:
- **User Store** (auth, profile, preferences)
- **UI Store** (modals, notifications, theme)
- **Filter Store** (product filters, search)

**8. Update Components**:
- Replace Context consumers with Zustand hooks
- Remove provider wrappers
- Update tests (remove provider wrappers)

**9. Establish Patterns**:
- Document store conventions in wiki
- Create code snippets (VSCode templates)
- Update onboarding guide

### Phase 4: Testing and Optimization (Week 4-5)

**10. Write Tests**:
```typescript
// __tests__/stores/useCartStore.test.ts
import { renderHook, act } from '@testing-library/react';
import { useCartStore } from '@/stores/useCartStore';

beforeEach(() => {
  useCartStore.setState({ items: [] });
});

it('adds item to cart', () => {
  const { result } = renderHook(() => useCartStore());
  act(() => result.current.addItem(mockItem));
  expect(result.current.items).toHaveLength(1);
});
```

**11. Performance Testing**:
- Load test: 1000 concurrent users (simulate peak traffic)
- Measure: State update latency (<100ms target)
- Memory profiling: Verify no memory leaks

**12. Documentation**:
- Update architecture docs (state management section)
- Create "Zustand Best Practices" wiki page
- Add code examples to team wiki

---

## Monitoring and Metrics

### Key Performance Indicators (KPIs)

**Performance Metrics**:
- State update latency: <100ms (P95)
- Re-render count: <50% of Context API baseline
- Bundle size: <300KB total (current: 180KB + 5KB Zustand = 185KB)

**Developer Metrics**:
- New store creation time: <30 minutes
- Bug fix time (state issues): <1 hour average
- Onboarding time (new developers): <3 days to productivity

**User Metrics**:
- Page load time: <2s
- Shopping cart interaction latency: <100ms
- Cart persistence success rate: >99.9%

### Monitoring Tools

**Performance Monitoring**:
- **Redux DevTools**: Track state changes, action history
- **React DevTools Profiler**: Measure re-render count
- **Lighthouse**: Monitor bundle size, performance scores

**Custom Logging**:
```typescript
// middleware/logger.ts
export const loggerMiddleware = (config) => (set, get, api) =>
  config(
    (...args) => {
      const before = get();
      set(...args);
      const after = get();
      console.log('[Zustand]', { before, after, timestamp: Date.now() });
    },
    get,
    api
  );
```

---

## Review Schedule

### 3 Months (January 2026)
**Review Criteria**:
- Are we meeting <100ms state update targets?
- Has boilerplate been reduced (vs Context API)?
- Are developers comfortable with Zustand patterns?

**Action Items**:
- Survey team: Satisfaction with Zustand (1-10 scale)
- Analyze performance metrics (Redux DevTools logs)
- Identify pain points (refactor if needed)

### 6 Months (April 2026)
**Review Criteria**:
- Has technical debt accumulated?
- Are store conventions consistent?
- Has bundle size remained under 300KB?

**Action Items**:
- Refactor common patterns into shared utilities
- Update documentation based on learnings
- Evaluate new Zustand features (middleware, utils)

### 12 Months (October 2026)
**Full ADR Review**:
- **Performance**: Still meeting targets as features grow?
- **Productivity**: Team still productive with Zustand?
- **Maintenance**: Code quality maintained?

---

## Related Documents

- **ADR-001: Database Selection** (PostgreSQL for server state)
- **ADR-002: ORM Selection** (Dapper for data access)
- **Tech Stack Documentation**: `devforgeai/specs/context/tech-stack.md` (lists Zustand as standard)
- **Dependencies Documentation**: `devforgeai/specs/context/dependencies.md` (approved Zustand packages)
- **Coding Standards**: `devforgeai/specs/context/coding-standards.md` (state management patterns)

---

## Approval and Sign-Off

**Approved By**:
- ✅ Frontend Lead (evaluated alternatives, benchmarked performance)
- ✅ Senior Frontend Developers (2) (validated developer experience)
- ✅ UX Engineer (confirmed no performance regressions)

**Dissent**: None

**Date Approved**: 2025-10-25

---

## References

### Benchmarks
- Internal benchmark: `docs/benchmarks/state-management-comparison-2025-10.md`
- [Zustand Performance Discussion](https://github.com/pmndrs/zustand/discussions)

### Documentation
- [Zustand Official Documentation](https://github.com/pmndrs/zustand)
- [Zustand Middleware Ecosystem](https://github.com/pmndrs/zustand#middleware)
- [React Query + Zustand Best Practices](https://tkdodo.eu/blog/combining-react-query-and-zustand)

### Team Resources
- Zustand tutorial: `docs/tutorials/zustand-getting-started.md`
- Migration guide: `docs/guides/context-to-zustand-migration.md`
- Testing patterns: `docs/patterns/zustand-testing.md`
