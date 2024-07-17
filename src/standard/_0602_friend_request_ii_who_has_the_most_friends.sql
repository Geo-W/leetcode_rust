select distinct top 1 t1.id,
                      case when rt.cnt is null then 0 else rt.cnt end +
                      case when at.cnt is null then 0 else at.cnt end as num
from (select requester_id as id from RequestAccepted union select accepter_id as id from RequestAccepted) t1
         left join (select requester_id as id, count(*) as cnt from RequestAccepted group by requester_id) rt
                   on t1.id = rt.id
         left join (select accepter_id as id, count(*) as cnt from RequestAccepted group by accepter_id) at
                   on t1.id = at.id
order by num desc
