# todo
- refactor
    - [ ] consider making ``Created`` auto-generated by dbms but modifiable after generation
    - [x] consider making ``Created`` immutable and only to be auto-generated by dbms
      - cancel
    - [x] (by, when(search)) to when(by, search)
- feat
    - [x] add
        - [x] item
            - [(-id, ..., tag_names?)] ->
            - INSERT IGNORE
        - [x] tag to item
            - [(item_id, tag_names)] ->
            - INSERT IGNORE
        - [x] date to item
            - [(item_id, dates)] ->
            - INSERT IGNORE
        - CANNOT BE ADDED
            - tag
            - date
    - [x] update
        - [x] item
            - [(id, ..., tag_names?)] ->
            - UPDATE IGNORE
        - CANNOT BE UPDATED
            - tag
            - date
    - [x] remove (by id only)
        - [x] item
            - id ->
            - DELETE IGNORE
        - [x] tag
            - id ->
            - DELETE IGNORE
        - [x] date
            - id ->
            - DELETE IGNORE
        - [x] tag from item
            - [(id, id)] ->
            - DELETE IGNORE
        - [x] date from item
            - [(id, id)] ->
            - DELETE IGNORE
    - [ ] select
        - [x] and|or
        - [x] dates
            - [x] before
            - [x] after
        - [ ] items
            - [ ] by name
                - tables
                    - [x] item
                - clauses
                    - [x] exact
                        - name = ?
                    - [x] substring
                        - name like ?
                    - [x] pattern
                        - name match ?
                    - [ ] fuzzy
                        - ???
            - [ ] by tag
                - tables
                    - [x] item
                    - [x] item_has_tag
                - clauses
                    - [x] exact
                        - tag_id = (select id from tag where name = ?)
                    - [x] substring
                        - tag_id = (select id from tag where name like ?)
                    - [x] pattern
                        - tag_id = (select id from tag where name match ?)
                    - [ ] fuzzy
                        - ???
            - [x] by date
        - [ ] tags
            - [ ] by name
                - [x] exact
                - [x] substring
                - [x] pattern
                - [ ] fuzzy
            - [ ] by item
                - [x] exact
                - [x] substring
                - [x] pattern
                - [ ] fuzzy
            - [x] by date
                - [x] equal
                - [x] before
                - [x] after

