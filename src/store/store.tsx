import { combine } from 'zustand/middleware'
import create from 'zustand'

const useStore = create(
  combine(
    { bears: 0 },
    (set) => ({ increase: (by: number) => set((state) => ({ bears: state.bears + by })) })
  ),
)

export default useStore