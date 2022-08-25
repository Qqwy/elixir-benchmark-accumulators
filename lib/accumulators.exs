Mix.install([:benchee])

n = 100

integers = Enum.to_list(1..n)
maps = Enum.map(1..n, &%{value: &1, nested_stuff: List.duplicate(:foo, 1000)})

Benchee.run(%{
  "reduce (integers)" => fn ->
    Enum.reduce(integers, 0, &+/2)
  end,
  "reduce (maps)" => fn ->
    Enum.reduce(maps, 0, fn el, acc -> el.value + acc end)
  end,
  "Agents (integers)" => fn ->
    {:ok, pid} = Agent.start(fn -> 0 end)

    Enum.each(integers, fn el ->
      :ok = Agent.update(pid, &(&1 + el))
    end)

    Agent.get(pid, & &1)
  end,
  "Agents (maps)" => fn ->
    {:ok, pid} = Agent.start(fn -> 0 end)

    Enum.each(maps, fn el ->
      :ok = Agent.update(pid, &(&1 + el.value))
    end)

    Agent.get(pid, & &1)
  end,
  ":counters (integers)" => fn ->
    counter = :counters.new(1, [])
    Enum.each(integers, &:counters.add(counter, 1, &1))
    :counters.get(counter, 1)
  end,
  "Process dictionary (integers)" => fn ->
    :erlang.put({__MODULE__, :accum}, 0)

    Enum.each(integers, fn el ->
      accum = :erlang.get({__MODULE__, :accum})
      :erlang.put({__MODULE__, :accum}, accum + el)
    end)
  end,
  "Process dictionary (maps)" => fn ->
    :erlang.put({__MODULE__, :accum}, 0)

    Enum.each(maps, fn el ->
      accum = :erlang.get({__MODULE__, :accum})
      :erlang.put({__MODULE__, :accum}, accum + el.value)
    end)
  end,
  "MVar (integers)" => fn ->
    var = MVar.new(accum, 0)

    Enum.each(integers, fn el ->
      MVar.set(MVar.get(accum) + el)
    end)
  end,
  "MVar (maps)" => fn ->
    var = MVar.new(accum, 0)

    Enum.each(maps, fn el ->
      MVar.set(MVar.get(accum + el.value))
    end)
  end
})
