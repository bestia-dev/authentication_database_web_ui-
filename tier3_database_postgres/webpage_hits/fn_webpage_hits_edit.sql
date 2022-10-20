select drop_function('webpage_hits_edit');

create function  public.webpage_hits_edit(
_id int)
returns table(id integer, webpage varchar(100), hit_count integer) 
language 'plpgsql'
as $body$
declare
begin

return query 
select w.id, w.webpage, w.hit_count
from webpage_hits w
where w.id=_id;

end; 
$body$;
