select user_id as buyer_id, join_date, case when t1.cnt is null then 0 else t1.cnt end as orders_in_2019
from Users
left join (select count(*) as cnt, buyer_id from Orders where year(order_date) = 2019 group by buyer_id ) t1 on user_id = buyer_id