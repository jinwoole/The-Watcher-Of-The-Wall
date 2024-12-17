import sqlite3

class Database:
    _instance = None

    def __new__(cls, db_path=None):
        if cls._instance is None:
            cls._instance = super(Database, cls).__new__(cls)
            cls._instance.db_path = db_path
            cls._instance.connection = None
            cls._instance.cursor = None
        return cls._instance

    def __init__(self, db_path=None):
        # __new__에서 이미 초기화했으므로 pass
        pass

    def connect(self):
        """
        데이터베이스에 연결합니다.
        연결이 없는 경우에만 연결을 설정합니다.
        """
        if not self.connection:
            try:
                self.connection = sqlite3.connect(self.db_path, check_same_thread=False)
                self.cursor = self.connection.cursor()
            except sqlite3.Error as e:
                print(f"데이터베이스 연결 오류: {e}")
                raise

    def close(self):
        """
        데이터베이스 연결을 종료합니다.
        연결이 존재하는 경우 닫고 상태를 초기화합니다.
        """
        if self.connection:
            try:
                self.connection.close()
                self.connection = None
                self.cursor = None
            except sqlite3.Error as e:
                print(f"데이터베이스 종료 오류: {e}")
                raise

    def execute(self, query, params=None):
        """
        쿼리를 실행하고 변경 사항을 커밋합니다.
        :param query: 실행할 SQL 쿼리
        :param params: 쿼리에 전달할 매개변수 (선택 사항)
        """
        self.connect()
        try:
            if params:
                self.cursor.execute(query, params)
            else:
                self.cursor.execute(query)
            self.connection.commit()
        except sqlite3.Error as e:
            print(f"쿼리 실행 오류: {e}")
            self.connection.rollback()
            raise

    def fetchone(self, query, params=None):
        """
        쿼리를 실행하고 단일 결과를 반환합니다.
        :param query: 실행할 SQL 쿼리
        :param params: 쿼리에 전달할 매개변수 (선택 사항)
        :return: 단일 결과 또는 None
        """
        self.connect()
        try:
            if params:
                self.cursor.execute(query, params)
            else:
                self.cursor.execute(query)
            return self.cursor.fetchone()
        except sqlite3.Error as e:
            print(f"쿼리 실행 오류: {e}")
            raise

    def fetch_all(self, query, params=None):
        """
        쿼리를 실행하고 모든 결과를 문자열 리스트로 반환합니다.
        :param query: 실행할 SQL 쿼리
        :param params: 쿼리에 전달할 매개변수 (선택 사항)
        :return: 문자열 리스트
        """
        self.connect()
        try:
            if params:
                self.cursor.execute(query, params)
            else:
                self.cursor.execute(query)
            results = self.cursor.fetchall()
            # 각 행의 두 번째 열(인덱스 1)부터 문자열로 변환하여 리스트에 추가
            return [str(row[1]) for row in results]
        except sqlite3.Error as e:
            print(f"쿼리 실행 오류: {e}")
            raise

    def create_table(self, query):
        """
        테이블을 생성하는 쿼리를 실행합니다.
        :param query: 테이블 생성 SQL 쿼리
        """
        self.connect()
        try:
            self.cursor.execute(query)
            self.connection.commit()
        except sqlite3.Error as e:
            print(f"테이블 생성 오류: {e}")
            self.connection.rollback()
            raise