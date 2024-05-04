add : fn (Int, Int): int
add = fn (a, b) -> a + b

add = fn (a: Int, b: Int): Int -> a + b

add: fn (Int, Int): Int = fn (a, b) -> a + b

add = fn (a: Int, b: Int) -> a + b

HttpMethod = enum {
  Get
  Post
  Put
  Patch
  Delete
}

Ip = enum {
  v4: tuple { Int, Int, Int, Int }
  v6: tuple { Int, Int, Int, Int }
}

Url = struct {
  schema: Str
  user: Str
  host: Ip
  port: Int
  path: Array(Str)
  query: Map(Str, Str)
  hash: Str
}

Ordering = enum {
  LT = -1
  EQ = 0
  GT = 1
}

// partition = T. fn (xs: Array(T), pred: fn (T): Bool): struct { yes: Array(T), no: Array(T) }

partition : T. fn (Array(T), fn (T): Bool): Int
partition = fn (xs, pred) -> {
  position := 0
  
  for (i := 0; i < xs.length; i += 1) {
    if (pred(xs[i])) {
      defer position += 1;
      tmp = xs[position]
      xs[position] = xs[i]
      xs[i] = tmp
    } 
  }

  return position
}

partition : T. fn (Array(T), fn (T): Bool): struct { yes: Array(T), no: Array(T) }
partition = fn (xs, pred) -> {
  copy = xs.copy()
  position = copy.partition(pred)

  return .{ 
    yes: copy.slice(0, position) 
    no: copy.slice(position + 1)
  }
}
