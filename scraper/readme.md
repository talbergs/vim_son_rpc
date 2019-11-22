Scrapes Zabbix WIKI. Collects API refernce into wikifiles.

These files will be used by other program that would compile into intelligent completion for son_rpc project.

Crawler begins in one of these pages:

https://www.zabbix.com/documentation/4.4/manual/api/reference?do=edit
https://www.zabbix.com/documentation/4.2/manual/api/reference?do=edit
https://www.zabbix.com/documentation/4.0/manual/api/reference?do=edit
https://www.zabbix.com/documentation/3.0/manual/api/reference?do=edit

From there it will follow all the domain links: (using quick parse of wiki text)
https://www.zabbix.com/documentation/4.4/manual/api/reference/history?do=edit

From there it will follow once again to all the method and object links: (using quick parse of wiki text)
https://www.zabbix.com/documentation/4.4/manual/api/reference/history/get?do=edit

Output produced:
```
4.4/
    index
    history/
        index
        get
        object
```
