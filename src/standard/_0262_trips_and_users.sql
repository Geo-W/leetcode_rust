select request_at as [Day], cast(cast(sum(case when [status] <> 'completed' then 1 else 0 end ) as decimal(10,0)) / cast(count([status]) as decimal(10,0)) as decimal(10,2)) as [Cancellation Rate]
from Trips
    left join Users a
on Trips.client_id = a.users_id
    left join Users b on Trips.driver_id = b.users_id
where a.banned = 'no' and b.banned = 'no' and request_at between '2013-10-01' and '2013-10-03'
group by request_at
