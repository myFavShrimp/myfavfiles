CREATE FUNCTION array_unique(arr anyarray)
RETURNS anyarray as $body$
    SELECT array( SELECT DISTINCT unnest($1) )
$body$ LANGUAGE 'sql';

CREATE FUNCTION uniquify_permissions_array_on_change()
    RETURNS TRIGGER 
    LANGUAGE PLPGSQL
    AS
$$
    BEGIN
	    NEW.permissions = array_unique(NEW.permissions);

	    RETURN NEW;
    END;
$$;

CREATE TRIGGER permission_changes
BEFORE UPDATE
ON platform_role
FOR EACH ROW
EXECUTE PROCEDURE uniquify_permissions_array_on_change();
 
CREATE TRIGGER permission_insert
BEFORE INSERT
ON platform_role
FOR EACH ROW
EXECUTE PROCEDURE uniquify_permissions_array_on_change();
 
 
CREATE TRIGGER permission_changes
BEFORE UPDATE
ON group_role
FOR EACH ROW
EXECUTE PROCEDURE uniquify_permissions_array_on_change();
 
CREATE TRIGGER permission_insert
BEFORE INSERT
ON group_role
FOR EACH ROW
EXECUTE PROCEDURE uniquify_permissions_array_on_change();
 
