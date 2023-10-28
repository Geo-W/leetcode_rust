SELECT Weather.id as Id
FROM Weather
         left join Weather a on Weather.recordDate = dateadd(day, 1, a.recordDate)
WHERE Weather.temperature > a.temperature